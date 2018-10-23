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
        let future = save_rx
            .take_while(move |_| future::ok(*is_saver_on))
            .for_each(|_value| {
                info!("Get some value!");
                Ok(())
            });
        executor.spawn(future);
    }

    pub fn stop_saver(&mut self) {
        warn!("Stopping saver!!!");
        *Arc::get_mut(&mut self.is_saver_on).unwrap() = false;
    }
}
