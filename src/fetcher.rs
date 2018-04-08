extern crate hyper;
extern crate hyper_tls;

use std::sync::Mutex;
use db::BotDb;
use fetcher::hyper::Error;
use fetcher::hyper::Response;
use fetcher::hyper::Uri;
use fetcher::hyper::client::HttpConnector;
use fetcher::hyper::{Client, StatusCode};
use fetcher::hyper_tls::HttpsConnector;
use futures::{future, Future, Stream};
use serde_json;
use serde_json::Value;
use tokio_core::reactor::{Core, Handle};

const BASE_URL: &'static str = "https://5etools.com/data";
const SPELLS: &'static str = "/spells";
const ITEMS: &'static str = "/items";
const BESTIARY: &'static str = "/bestiary";
const INDEX: &'static str = "index";
const EXTENSION: &'static str = ".json";

pub struct Fetcher {
    client: Client<HttpsConnector<HttpConnector>>,
    core: Mutex<Core>,
    db: Mutex<BotDb>,
}

impl Fetcher {
    pub fn init(core: Core, handle: &Handle, db: BotDb) -> Fetcher {
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle).unwrap())
            .build(&handle);
        Fetcher {
            client: client,
            core: Mutex::new(core),
            db: Mutex::new(db),
        }
    }

    pub fn fetch(&self) {
        // let urls: Vec<&str> = vec![SPELLS, ITEMS, BESTIARY];
        let urls: Vec<&str> = vec![ITEMS];
        for url_part in urls {
            let uri = BASE_URL.to_string() + url_part + EXTENSION;
            let uri = uri.parse().unwrap();
            println!("{:?}", uri);

            let items: Vec<Value> = Vec::new();

            let work = self.client.get(uri).and_then(|res| {
                println!("Response: {}", res.status());

                match res.status() {
                    // StatusCode::Ok => future::Either::A(self.process_ok(res)),
                    // StatusCode::NotFound => future::Either::B(),
                    StatusCode::Ok => self.process_ok(res),
                    _ => panic!("Bad url"),
                }
            });
            self.core.lock().unwrap().run(work).unwrap();
        }
    }

    fn process_ok(&self, res: Response) -> Box<Future<Item = (), Error = Error>> {
        let future = res.body().concat2().and_then(|body| {
            let v: Value = serde_json::from_slice(&body).unwrap();
            // self.db.lock().unwrap().save(v);
            println!("Ok!");
            Ok(())
        });
        Box::new(future)
    }
}
