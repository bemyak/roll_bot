use serde_json;
use unqlite::{
    document::{Jx9, UnQLiteVm, Value}, UnQLite,
};
use util;

const INSERT_SCRIPT: &str = include_str!("jx9/insert.jx9");
const SEARCH_SCRIPT: &str = include_str!("jx9/search.jx9");

pub fn init() -> UnQLite {
    UnQLite::create_in_memory()
}

pub fn save(db: &UnQLite, value: String) {
    let mut insert = prepare_script(db, INSERT_SCRIPT);
    insert.add_variable("entry", Value::string(value)).unwrap();
    insert.exec_void().unwrap()
}

pub fn search(db: &UnQLite, value: &str) -> serde_json::Value {
    let mut search = prepare_script(db, SEARCH_SCRIPT);
    search.add_variable("params", Value::string(value)).unwrap();
    let result = search.exec().unwrap().unwrap();
    util::convert(&result)
}

fn prepare_script(db: &UnQLite, script: &str) -> UnQLiteVm {
    let vm = db.compile(script).unwrap();
    vm.output_to_stdout().unwrap();
    vm
}
