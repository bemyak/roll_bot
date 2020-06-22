#![allow(dead_code)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;

mod db;
mod fetch;
mod format;
mod telegram;

use std::error::Error;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use prometheus::{Encoder, HistogramVec, IntCounterVec, IntGaugeVec, TextEncoder};
#[allow(unused_imports)]
use tokio::task;
use tokio::time;

use db::DndDatabase;

pub const PROJECT_URL: &'static str = "https://gitlab.com/bemyak/roll_bot";
pub const METRICS_PORT: u16 = 9889;
pub const METRICS_ENDPOINT: &'static str = "/metrics";

lazy_static! {
    pub static ref MESSAGE_COUNTER: IntCounterVec = {
        let opts = opts!("message_counter", "Displays number of messages");
        register_int_counter_vec!(opts, &["user_id", "chat_type", "command"]).unwrap()
    };
    pub static ref ERROR_COUNTER: IntCounterVec = {
        let opts = opts!("error_counter", "Number of errors occurred");
        register_int_counter_vec!(opts, &["command"]).unwrap()
    };
    pub static ref REQUEST_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "request_duration_seconds",
        "Request duration histogram",
        &["command"]
    )
    .unwrap();
    pub static ref COLLECTION_ITEM_GAUGE: IntGaugeVec = {
        let opts = opts!(
            "collection_item_count",
            "Number of items in each collection"
        );
        register_int_gauge_vec!(opts, &["collection"]).unwrap()
    };
    pub static ref COLLECTION_TIMESTAMP_GAUGE: IntGaugeVec = {
        let opts = opts!(
            "collection_update_timestamp",
            "Last database update, UNIX timestamp"
        );
        register_int_gauge_vec!(opts, &["collection"]).unwrap()
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let log_config = simplelog::ConfigBuilder::new()
        .add_filter_allow_str("roll_bot")
        .build();
    simplelog::SimpleLogger::init(simplelog::LevelFilter::Trace, log_config)?;

    task::spawn(async move { serve_metrics().await });

    // Use this while testing to avoid unnecessary loading 5e.tools
    // let db = DndDatabase::new("./test_data/roll_bot.ejdb")?;

    // Uncomment this when ready for production use
    let db = DndDatabase::new("./roll_bot.ejdb")?;
    let fetch_db = db.clone();
    task::spawn(async move {
        fetch_job(fetch_db).await;
    });

    let bot = telegram::Bot::new(db.clone()).await?;

    bot.start().await?;

    Ok(())
}

async fn fetch_job(mut db: DndDatabase) {
    let mut interval = time::interval(Duration::from_secs(60 * 60 * 24));

    loop {
        interval.tick().await;

        let result = fetch::fetch().await;
        match result {
            Ok(data) => {
                for (collection, items) in data {
                    db.save_collection(items, collection).unwrap_or_else(|err| {
                        error!("Error occurred while saving data to DB: {}", err)
                    });
                }
            }
            Err(err) => error!("Error occurred while fetching data: {}", err),
        }
    }
}

async fn serve_metrics() -> Result<(), Box<dyn Error + Send + Sync>> {
    let metrics_addr = ([0, 0, 0, 0], METRICS_PORT).into();
    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(metrics)) });
    info!(
        "Serving metrics at http://{}{}",
        metrics_addr, METRICS_ENDPOINT
    );
    let server = Server::bind(&metrics_addr).serve(service);
    server.await?;
    Ok(())
}

async fn metrics(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, METRICS_ENDPOINT) => {
            let encoder = TextEncoder::new();
            let metric_families = prometheus::gather();
            let mut buffer = vec![];
            encoder.encode(&metric_families, &mut buffer).unwrap();
            Ok(Response::new(Body::from(buffer)))
        }
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

pub fn get_unix_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
