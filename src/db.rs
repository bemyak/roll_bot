use std::clone::Clone;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

use ejdb::bson;
use ejdb::bson::Bson;
use ejdb::query::{Q, QH};
use ejdb::Database;
use ejdb::Result as EjdbResult;
use serde_json::Value as JsonValue;
use telegram_bot::{Message, MessageChat, MessageText};

use crate::get_unix_time;

// System table should start with "_", so they will not be treated like D&D data collections
const LOG_COLLECTION_NAME: &'static str = "_log";

pub struct DndDatabase(Arc<Mutex<Inner>>);

struct Inner {
    db: Database,
    timestamp: Instant,
}

impl Clone for DndDatabase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl DndDatabase {
    pub fn new(path: &str) -> Result<DndDatabase, Box<dyn Error>> {
        let ejdb = Database::open(path)?;
        // Create log index
        {
            let coll = ejdb.collection(LOG_COLLECTION_NAME)?;
            coll.index("timestamp").number().set()?;
            coll.index("user_id").number().set()?;
        }

        Ok(DndDatabase(Arc::new(Mutex::new(Inner {
            db: ejdb,
            timestamp: Instant::now(),
        }))))
    }

    pub fn save_collection(
        &mut self,
        json: Vec<JsonValue>,
        collection: &str,
    ) -> Result<(), Box<dyn Error>> {
        let bs: Bson = serde_json::Value::Array(json).into();
        let arr = bs.as_array().ok_or(bson::DecoderError::Unknown(
            format!("{} field is not an array", collection).to_owned(),
        ))?;
        let mut inner = self.0.try_lock().unwrap();
        inner.timestamp = Instant::now();
        inner.db.drop_collection(collection, false)?;
        let coll = inner.db.collection(collection)?;
        coll.index("name").string(false).set()?;
        arr.into_iter()
            .filter_map(|elem| elem.as_document())
            .for_each(|elem| {
                let res = coll.save(elem);
                if let Err(e) = res {
                    error!("Failed to save document: {}", e)
                }
            });
        Ok(())
    }

    pub fn get_update_timestamp(&self) -> Instant {
        let inner = self.0.try_lock().unwrap();
        inner.timestamp
    }

    pub fn get_metadata(&self) -> Result<ejdb::meta::DatabaseMetadata, Box<dyn Error>> {
        let inner = self.0.try_lock().unwrap();
        Ok(inner.db.get_metadata()?)
    }

    // This is terribly inefficient, but upstream EJDB bindings does not implement distinct queries :(
    pub fn get_all_massages(&self) -> Result<Vec<LogMessage>, Box<dyn Error>> {
        let inner = self.0.try_lock().unwrap();
        let coll = inner.db.collection(LOG_COLLECTION_NAME)?;
        Ok(coll
            .query(Q.empty(), QH.empty())
            .find()?
            .filter_map(Result::ok)
            .map(|doc| doc.try_into())
            .filter_map(Result::ok)
            .collect())
    }

    pub fn get_cache(&self) -> HashMap<String, Vec<String>> {
        let inner = self.0.try_lock().unwrap();
        let collections = DndDatabase::list_collections(&inner.db);
        let mut result = HashMap::with_capacity(collections.len());
        collections.into_iter().for_each(|collection| {
            let items = DndDatabase::list_items(&inner.db, &collection).unwrap_or_default();
            result.insert(collection, items);
        });
        result
    }

    fn list_collections(db: &Database) -> Vec<String> {
        db.get_metadata()
            .unwrap()
            .collections()
            .map(|coll| coll.name().to_owned())
            .filter(|coll| !coll.starts_with("_"))
            .collect::<Vec<_>>()
    }

    fn list_items(db: &Database, collection: &str) -> Result<Vec<String>, Box<dyn Error>> {
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
    ) -> Result<bson::Document, Box<dyn Error>> {
        let inner = self.0.try_lock().unwrap();
        let coll = inner.db.collection(collection)?;
        coll.query(Q.field("name").case_insensitive().eq(item_name), QH.empty())
            .find_one()?
            .ok_or(Box::new(bson::DecoderError::Unknown(
                "Not found".to_owned(),
            )))
    }

    pub fn log_message(
        &self,
        message: &Message,
        response: &Result<Option<String>, Box<dyn Error>>,
    ) {
        match self.try_log_message(message, response) {
            Ok(_) => {}
            Err(err) => error!("Failed to save message to db: {}", err),
        }
    }

    fn try_log_message(
        &self,
        message: &Message,
        response: &Result<Option<String>, Box<dyn Error>>,
    ) -> Result<(), Box<dyn Error>> {
        let inner = self.0.try_lock().unwrap();
        let coll = inner.db.collection(LOG_COLLECTION_NAME)?;

        let user_id: i64 = message.from.id.into();
        let chat_type = chat_type_to_string(&message.chat);
        let request = message.text().unwrap_or_default();

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

        coll.save(bson! {
            "timestamp" => timestamp,
            "user_id" => user_id,
            "chat_type" => chat_type,
            "request" => request,
            "response" => response
        })?;

        Ok(())
    }
}

pub struct LogMessage {
    pub timestamp: u64,
    pub user_id: i64,
    pub chat_type: String,
    pub response: Option<String>,
}

fn chat_type_to_string(chat_type: &MessageChat) -> &'static str {
    match chat_type {
        MessageChat::Private(_) => "Private",
        MessageChat::Group(_) => "Group",
        MessageChat::Supergroup(_) => "Supergroup",
        MessageChat::Unknown(_) => "Unknown",
    }
}

impl TryFrom<bson::ordered::OrderedDocument> for LogMessage {
    type Error = bson::ValueAccessError;
    fn try_from(value: bson::ordered::OrderedDocument) -> Result<Self, Self::Error> {
        let timestamp = value.get_i64("timestamp")?;
        Ok(LogMessage {
            timestamp: timestamp as u64,
            user_id: value.get_i64("user_id")?,
            chat_type: value.get_str("chat_type")?.to_owned(),
            response: value.get_str("response").map(str::to_owned).ok(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::DndDatabase;
    use crate::format::format_document;

    use simplelog::*;

    fn init() {
        let _ = TestLogger::init(LevelFilter::Trace, Config::default());
    }

    #[test]
    fn test_get_cache() {
        init();
        let db = DndDatabase::new(get_db_path()).unwrap();
        let cache = db.get_cache();
        info!("{:?}", cache);
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
        let i = db.get_item("spell", "Fireball").unwrap();
        info!("{:#?}", i);
        info!("{}", format_document(i));
    }

    fn get_db_path() -> &'static str {
        "./test_data/roll_bot.db"
    }
}
