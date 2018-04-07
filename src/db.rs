extern crate unqlite;

use std::sync::Mutex;
use db::unqlite::{UnQLite, document::{Jx9, UnQLiteVm}};
use serde_json;

pub struct BotDb {
    _db: UnQLite,
    insert: Mutex<UnQLiteVm>,
}

impl BotDb {
    pub fn init() -> BotDb {
        let db = UnQLite::create_in_memory();

        let insert = include_str!("jx9/insert.jx9");
        let insert = db.compile(insert).unwrap();
        insert.output_to_stdout().unwrap();

        return BotDb {
            _db: db,
            insert: Mutex::new(insert),
        };
    }

    pub fn save(&self, value: serde_json::Value) {
        self.insert.lock().unwrap().exec().unwrap();
    }
}
