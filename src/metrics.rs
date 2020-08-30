use std::error::Error;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use prometheus::{Encoder, HistogramVec, IntCounterVec, IntGaugeVec, TextEncoder};

pub const METRICS_PORT: u16 = 9889;
pub const METRICS_ENDPOINT: &str = "/metrics";

lazy_static! {
    // Metric collectors
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

pub async fn serve_metrics() -> Result<(), Box<dyn Error + Send + Sync>> {
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
