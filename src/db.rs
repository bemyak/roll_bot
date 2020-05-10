use std::clone::Clone;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;

use ejdb::bson;
use ejdb::query::{Q, QH};
use ejdb::Database;
use ejdb::Result as EjdbResult;
use serde_json::Value as JsonValue;

pub struct DndDatabase(Arc<Mutex<Database>>);

impl Clone for DndDatabase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl DndDatabase {
    pub fn new(path: &str) -> Result<DndDatabase, Box<dyn Error>> {
        let ejdb = Database::open(path)?;
        Ok(DndDatabase(Arc::new(Mutex::new(ejdb))))
    }

    pub fn save_collection(&self, json: JsonValue, collection: &str) -> Result<(), Box<dyn Error>> {
        let bs: bson::Bson = json.into();
        let doc = bs.as_document().ok_or(bson::DecoderError::Unknown(
            format!("Not a document, but a {:?}", bs.element_type()).to_owned(),
        ))?;
        let f = doc.get(collection).ok_or(bson::DecoderError::Unknown(
            format!("Does not have {} field", collection).to_owned(),
        ))?;
        let arr = f.as_array().ok_or(bson::DecoderError::Unknown(
            format!("{} field is not an array", collection).to_owned(),
        ))?;
        let db = self.0.try_lock().unwrap();
        let coll = db.collection(collection)?;
        coll.index("name").number().string(false).set()?;
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

    pub fn get_stats(&self) -> String {
        let db = self.0.try_lock().unwrap();
        db.get_metadata()
            .unwrap()
            .collections()
            .map(|col| format!("{}: {} records", col.name(), col.records()))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn list_collection(&self, collection: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let db = self.0.try_lock().unwrap();
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
        let db = self.0.try_lock().unwrap();
        let coll = db.collection(collection)?;
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
    fn test_list_collection() {
        init();
        let db = DndDatabase::new(get_db_path()).unwrap();
        info!("{:?}", db.list_collection("item").unwrap());
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
