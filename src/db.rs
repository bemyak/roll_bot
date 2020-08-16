use std::clone::Clone;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::{mem, time::Instant};

use ejdb::bson;
use ejdb::bson::Bson;
use ejdb::query::{Q, QH};
use ejdb::Database;
use ejdb::Result as EjdbResult;
use serde_json::Value as JsonValue;
use simsearch::{SearchOptions, SimSearch};

use crate::get_unix_time;
use crate::{
    collection::{CollectionName, COLLECTION_NAMES},
    metrics::{COLLECTION_ITEM_GAUGE, COLLECTION_TIMESTAMP_GAUGE},
};

// System table should start with an underscore, so they will not be treated like D&D data collections
const LOG_COLLECTION_NAME: &'static str = "_log";

pub struct DndDatabase {
    pub cache: Arc<Mutex<HashMap<CollectionName, SimSearch<CollectionName>>>>,
    inner: Arc<Mutex<Inner>>,
}

struct Inner {
    db: Database,
    timestamp: Instant,
}

impl Clone for DndDatabase {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            cache: self.cache.clone(),
        }
    }
}

impl DndDatabase {
    pub fn new(path: &str) -> Result<DndDatabase, ejdb::Error> {
        let ejdb = Database::open(path)?;
        // Create log index
        {
            let coll = ejdb.collection(LOG_COLLECTION_NAME)?;
            coll.index("timestamp").number().set()?;
            coll.index("user_id").number().set()?;
        }

        let inner = Inner {
            db: ejdb,
            timestamp: Instant::now(),
        };

        Ok(Self {
            cache: Arc::new(Mutex::new(inner.get_cache())),
            inner: Arc::new(Mutex::new(inner)),
        })
    }

    pub fn save_collection(
        &mut self,
        json: Vec<JsonValue>,
        collection: &str,
    ) -> Result<(), ejdb::Error> {
        info!("Saving {}, {}", collection, json.len());
        let bs: Bson = serde_json::Value::Array(json).into();
        let arr = bs.as_array().ok_or(bson::DecoderError::Unknown(
            format!("{} field is not an array", collection).to_owned(),
        ))?;
        let mut inner = self.inner.lock().unwrap();
        inner.timestamp = Instant::now();
        inner.db.drop_collection(collection, true)?;
        let coll = inner.db.collection(collection)?;
        arr.iter()
            .filter_map(|elem| elem.as_document())
            .for_each(|elem| {
                let res = coll.save(elem);
                if let Err(e) = res {
                    error!("Failed to save document: {}", e)
                }
            });
        coll.index("name").string(false).set()?;
        coll.index("abbreviation").string(true).set()?;
        coll.index("appliesTo").string(true).set()?;

        COLLECTION_TIMESTAMP_GAUGE
            .with_label_values(&[collection])
            .set(get_unix_time() as i64);

        COLLECTION_ITEM_GAUGE
            .with_label_values(&[collection])
            .set(arr.len() as i64);

        Ok(())
    }

    pub fn update_cache(&self) {
        let inner = self.inner.lock().unwrap();
        let new_cache = inner.get_cache();
        let mut cache = self.cache.lock().unwrap();
        *cache = new_cache;
    }

    pub fn get_update_timestamp(&self) -> Instant {
        let inner = self.inner.lock().unwrap();
        inner.timestamp
    }

    pub fn get_metadata(&self) -> Result<ejdb::meta::DatabaseMetadata, ejdb::Error> {
        let inner = self.inner.lock().unwrap();
        Ok(inner.db.get_metadata()?)
    }

    // This is terribly inefficient, but upstream EJDB bindings does not implement distinct queries :(
    pub fn get_all_massages(&self) -> Result<Vec<LogMessage>, ejdb::Error> {
        let inner = self.inner.lock().unwrap();
        let coll = inner.db.collection(LOG_COLLECTION_NAME)?;
        Ok(coll
            .query(Q.empty(), QH.empty())
            .find()?
            .filter_map(Result::ok)
            .map(|doc| doc.try_into())
            .filter_map(Result::ok)
            .collect())
    }

    fn list_collections(db: &Database) -> Vec<String> {
        db.get_metadata()
            .unwrap()
            .collections()
            .map(|coll| coll.name().to_owned())
            .filter(|coll| !coll.starts_with("_"))
            .collect::<Vec<_>>()
    }

    fn list_items(db: &Database, collection: &str) -> Result<Vec<String>, ejdb::Error> {
        let coll = db.collection(collection)?;
        let res = coll
            .query(Q.empty(), QH.field("name").include())
            .find()?
            .collect::<EjdbResult<Vec<bson::Document>>>()?;
        Ok(res
            .iter()
            .filter_map(|doc| doc.get("name"))
            .filter_map(|doc| doc.as_str())
            .map(|doc| doc.to_owned())
            .collect::<Vec<String>>())
    }

    pub fn get_item(
        &self,
        collection: &str,
        item_name: &str,
    ) -> Result<Option<bson::Document>, ejdb::Error> {
        let inner = self.inner.lock().unwrap();
        let coll = inner.db.collection(collection)?;
        coll.query(Q.field("name").case_insensitive().eq(item_name), QH.empty())
            .find_one()
    }

    pub fn find_one_by(
        &self,
        collection: &str,
        field: &str,
        value: &str,
    ) -> Result<Option<bson::Document>, ejdb::Error> {
        let inner = self.inner.lock().unwrap();
        let coll = inner.db.collection(collection)?;
        coll.query(Q.field("abbreviation").eq(value), QH.empty())
            .find_one()
    }

    pub fn log_message(
        &self,
        user_id: i64,
        chat_type: &str,
        request: String,
        response: &Result<Option<String>, Box<dyn Error>>,
        latency: u64,
    ) {
        match self.try_log_message(user_id, chat_type, request, response, latency) {
            Ok(_) => {}
            Err(err) => error!("Failed to save message to db: {}", err),
        }
    }

    fn try_log_message(
        &self,
        user_id: i64,
        chat_type: &str,
        request: String,
        response: &Result<Option<String>, Box<dyn Error>>,
        latency: u64,
    ) -> Result<(), ejdb::Error> {
        let inner = self.inner.lock().unwrap();
        let coll = inner.db.collection(LOG_COLLECTION_NAME)?;

        let mut default_response = String::new();
        let response = match response {
            Ok(response) => match response {
                None => &default_response,
                Some(response) => response,
            },
            Err(err) => {
                let mut err = format!("{}", err);
                std::mem::swap(&mut default_response, &mut err);
                &default_response
            }
        };

        let timestamp = get_unix_time();

        let bson = bson! {
            "timestamp" => timestamp,
            "user_id" => user_id,
            "chat_type" => chat_type,
            "request" => request,
            "response" => response,
            "latency" => latency
        };

        trace!("{}", bson);

        coll.save(bson)?;

        Ok(())
    }
}

impl Inner {
    fn get_cache(&self) -> HashMap<CollectionName, SimSearch<CollectionName>> {
        let mut result: HashMap<CollectionName, SimSearch<CollectionName>> =
            HashMap::with_capacity(COLLECTION_NAMES.len());
        COLLECTION_NAMES
            .iter()
            .for_each(|collection: &CollectionName| {
                let collection = *collection;
                let mut engine = SimSearch::new_with(get_search_options());
                DndDatabase::list_items(&self.db, &collection)
                    .unwrap_or_default()
                    .into_iter()
                    .for_each(|item| {
                        engine.insert(collection, &item);
                    });

                result.insert(collection, engine);
            });
        result
    }
}

pub fn get_search_options() -> SearchOptions {
    SearchOptions::new()
        .case_sensitive(false)
        .stop_whitespace(false)
        .threshold(0.85)
}

pub struct LogMessage {
    pub timestamp: u64,
    pub user_id: i64,
    pub chat_type: String,
    pub response: Option<String>,
    pub latency: u64,
}

impl TryFrom<bson::ordered::OrderedDocument> for LogMessage {
    type Error = bson::ValueAccessError;
    fn try_from(value: bson::ordered::OrderedDocument) -> Result<Self, Self::Error> {
        let timestamp = value.get_i64("timestamp")?;
        let latency = value.get_i64("latency")?;
        Ok(LogMessage {
            timestamp: timestamp as u64,
            user_id: value.get_i64("user_id")?,
            chat_type: value.get_str("chat_type")?.to_owned(),
            response: value.get_str("response").map(str::to_owned).ok(),
            latency: latency as u64,
        })
    }
}

#[cfg(test)]
mod test {
    use super::DndDatabase;
    use crate::format::Entry;

    use simplelog::*;

    fn init() {
        let _ = TestLogger::init(LevelFilter::Trace, Config::default());
    }

    #[test]
    fn test_get_cache() {
        init();
        let db = DndDatabase::new(get_db_path()).unwrap();
        let cache = db.cache.lock().unwrap();
        // info!("{:?}", cache);
        assert!(cache.len() > 0);
    }

    #[test]
    fn test_get_stats() {
        init();
        let db = DndDatabase::new(get_db_path()).unwrap();
        info!("{:?}", db.get_metadata().unwrap());
    }

    #[test]
    fn test_get_item() {
        init();
        let db = DndDatabase::new(get_db_path()).unwrap();
        let i = db.get_item("spell", "Fireball").unwrap().unwrap();
        info!("{:#?}", i);
        info!("{}", i.format());
    }

    fn get_db_path() -> &'static str {
        "./test_data/roll_bot.ejdb"
    }
}
