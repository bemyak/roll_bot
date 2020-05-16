use std::clone::Clone;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

use ejdb::bson;
use ejdb::query::{Q, QH};
use ejdb::Database;
use ejdb::Result as EjdbResult;
use serde_json::Value as JsonValue;

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
        let bs: bson::Bson = serde_json::Value::Array(json).into();
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

    pub fn get_timestamp(&self) -> Instant {
        let inner = self.0.try_lock().unwrap();
        inner.timestamp
    }

    pub fn get_stats(&self) -> String {
        let inner = self.0.try_lock().unwrap();
        inner
            .db
            .get_metadata()
            .unwrap()
            .collections()
            .map(|col| format!("{}: {} records", col.name(), col.records()))
            .collect::<Vec<_>>()
            .join("\n")
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
            .find()
            .unwrap()
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
        info!("{}", db.get_stats());
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
