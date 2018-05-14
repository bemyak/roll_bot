extern crate hyper;
extern crate hyper_tls;

use db::BotDb;
use fetcher::hyper::client::HttpConnector;
use fetcher::hyper::Error;
use fetcher::hyper::Response;
use fetcher::hyper::{Client, StatusCode};
use fetcher::hyper_tls::HttpsConnector;
use futures::{Future, Stream};
use serde_json;
use serde_json::Value::Object;
use std::sync::Mutex;
use tokio_core::reactor::Core;

const BASE_URL: &str = "https://5etools.com/data";
const SPELLS: &str = "/spells";
const ITEMS: &str = "/items";
const BESTIARY: &str = "/bestiary";
const INDEX: &str = "/index.json";
const EXTENSION: &str = ".json";

pub struct Fetcher<'a> {
    client: Client<HttpsConnector<HttpConnector>>,
    core: Mutex<Core>,
    db: &'a BotDb,
}

impl<'a> Fetcher<'a> {
    pub fn init(core: Core, db: &'a BotDb) -> Fetcher<'a> {
        let handle = &core.handle();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, handle).unwrap())
            .build(handle);
        Fetcher {
            client,
            core: Mutex::new(core),
            db,
        }
    }

    pub fn fetch(&self) {
        // let urls: Vec<&str> = vec![SPELLS, ITEMS, BESTIARY];
        let urls: Vec<&str> = vec![ITEMS, SPELLS, BESTIARY];
        for url_part in urls {
            let uri = BASE_URL.to_string() + url_part + EXTENSION;
            let uri = uri.parse().unwrap();
            println!("Downloading: {}", uri);
            let work = self.client.get(uri).and_then(|res| {
                match res.status() {
                    StatusCode::Ok => self.process_ok(res),
                    StatusCode::NotFound => self.process_not_found(url_part),
                    // TODO: somehow process error
                    _ => panic!(),
                }
            });
            let result = self.core.lock().unwrap().run(work).unwrap();
            match result {
                Ok(value) => self.db.save(value),
                Err(value) => {
                    if let Object(map) = serde_json::from_str(&value).unwrap() {
                        for file in map.values() {
                            let uri =
                                BASE_URL.to_string() + url_part + "/" + file.as_str().unwrap();
                            println!("Downloading: {}", uri);
                            let uri = uri.parse().unwrap();
                            let work = self.client.get(uri).and_then(|res| {
                                match res.status() {
                                    StatusCode::Ok => self.process_ok(res),
                                    // TODO: somehow process error
                                    _ => panic!(),
                                }
                            });
                            let value = self.core.lock().unwrap().run(work).unwrap().unwrap();
                            self.db.save(value);
                        }
                    }
                }
            }
        }
    }

    fn process_ok(
        &self,
        res: Response,
    ) -> Box<Future<Item = Result<String, String>, Error = Error>> {
        let future = res.body().concat2().and_then(|body| {
            let s = String::from_utf8(body.to_vec()).unwrap();
            Ok(Ok(s))
        });
        Box::new(future)
    }

    fn process_not_found(
        &self,
        url_part: &str,
    ) -> Box<Future<Item = Result<String, String>, Error = Error>> {
        let uri = BASE_URL.to_string() + url_part + INDEX;
        let uri = uri.parse().unwrap();
        let future = self.client
            .get(uri)
            .and_then(|res| {
                if res.status() == StatusCode::Ok {
                    res.body().concat2()
                } else {
                    panic!("Bad url: {}", res.status())
                }
            })
            .and_then(|body| {
                let s = String::from_utf8(body.to_vec()).unwrap();
                Ok(Err(s))
            });
        Box::new(future)
    }
}
