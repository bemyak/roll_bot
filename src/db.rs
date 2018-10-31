use ejdb::bson::{Bson, Document};
use ejdb::Database;
use futures::{future, sync::mpsc::Receiver, Stream};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use tokio::runtime::TaskExecutor;

pub struct Storage {
    db: Arc<Mutex<Database>>,
    is_saver_on: Arc<bool>,
}

impl<'a> Storage {
    pub fn init() -> Storage {
        let db = Arc::new(Mutex::new(Database::open("db.ejdb").unwrap()));

        return Self {
            db: db,
            is_saver_on: Arc::new(false),
        };
    }

    pub fn start_saver(&mut self, executor: &'a TaskExecutor, save_rx: Receiver<Value>) {
        *Arc::get_mut(&mut self.is_saver_on).unwrap() = true;
        let is_saver_on = self.is_saver_on.clone();
        let db = self.db.clone();
        let future = save_rx
            .take_while(move |_| future::ok(*is_saver_on))
            .for_each(move |value| {
                let thread_db = db.lock().unwrap();
                let doc = convert_value_to_document(&value);
                doc.iter().for_each(move |(key, value)| {
                    if let Bson::Document(doc) = value {
                        thread_db
                            .collection(key.clone().as_str())
                            .unwrap()
                            .save(doc)
                            .unwrap();
                    } else {
                        panic!("afasdf")
                    }
                });
                // db.collection()
                // info!("Get some value!");
                Ok(())
            });
        executor.spawn(future);
    }

    pub fn stop_saver(&mut self) {
        warn!("Stopping saver!!!");
        *Arc::get_mut(&mut self.is_saver_on).unwrap() = false;
    }
}

pub fn convert_value_to_document(value: &Value) -> Document {
    match value {
        Value::Object(map) => {
            let mut result = Document::new();
            map.iter().for_each(|(key, value)| {
                result
                    .insert(key.clone(), convert_value_to_bson(value))
                    .unwrap();
            });
            result
        }
        _ => panic!("bad json!"),
    }
}

pub fn convert_value_to_bson(value: &Value) -> Bson {
    match value {
        Value::Object(map) => {
            let mut result = Document::new();
            map.iter().for_each(|(key, value)| {
                result
                    .insert(key.clone(), convert_value_to_bson(value))
                    .unwrap();
            });
            Bson::Document(result)
        }

        Value::Bool(boool) => Bson::Boolean(boool.clone()),
        Value::Number(n) => {
            if n.is_i64() {
                return Bson::I64(n.as_i64().unwrap().clone());
            } else if n.is_f64() {
                return Bson::FloatingPoint(n.as_f64().unwrap().clone());
            } else {
                return Bson::FloatingPoint(n.as_f64().unwrap().clone());
            }
        }
        Value::String(string) => Bson::String(string.clone()),
        Value::Array(array) => {
            Bson::Array(array.iter().map(|val| convert_value_to_bson(val)).collect())
        }
        Value::Null => Bson::Null,
    }
}

// pub fn convert(bson: &Document) -> Value {
//     let result = Map::new();
//     for (key, value) in bson.iter() {
//         result.insert(key.to_string(), convert_bson(value));
//     }
//     return Value::Object(result);
// }

// fn convert_bson(bson: &Bson) -> Value {
//     match bson {
//         Bson::FloatingPoint(f) => Value::Number(Number::from_f64(f.clone()).unwrap()),
//     }
// }
