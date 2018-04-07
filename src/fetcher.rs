extern crate hyper;
extern crate hyper_tls;

use db::BotDb;
use fetcher::hyper::Error;
use fetcher::hyper::Response;
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
const EXTENSION: &'static str = ".json";

pub fn fetch(core: &mut Core, handle: &Handle, db: &BotDb) {
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);
    let urls: Vec<&str> = vec![SPELLS, ITEMS, BESTIARY];
    for url_part in urls {
        let uri = BASE_URL.to_string() + url_part + EXTENSION;
        let uri = uri.parse().unwrap();
        println!("{:?}", uri);

        let items: Vec<Value> = Vec::new();

        let work = client.get(uri).and_then(|res| {
            println!("Response: {}", res.status());

            match res.status() {
                StatusCode::Ok => future::Either::A(process_ok(res)),
                _ => future::Either::B(Error::Status),
            };
            Ok(())
        });
        core.run(work).unwrap();
    }
}

fn process_ok(res: Response) -> Box<Future<Item = Value, Error = Error>> {
    let future = res.body().concat2().and_then(move |body| {
        let v: Value = serde_json::from_slice(&body).unwrap();
        Ok(v)
    });
    Box::new(future)
}
