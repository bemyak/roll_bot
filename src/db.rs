extern crate unqlite;

use db::unqlite::{
    document::{Jx9, UnQLiteVm, Value}, UnQLite,
};
use std::sync::Mutex;

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

        BotDb {
            _db: db,
            insert: Mutex::new(insert),
        }
    }

    pub fn save(&self, value: String) {
        let mut insert = self.insert.lock().unwrap();
        insert.add_variable("entry", Value::string(value)).unwrap();
        insert.exec().unwrap();
    }
}
