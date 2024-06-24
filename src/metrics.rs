use std::{error::Error, net::SocketAddr};

use http_body_util::Full;
use hyper::{
	body::{Bytes, Incoming},
	server::conn::http1,
	service::service_fn,
	Method, Request, Response, StatusCode,
};
use hyper_util::rt::TokioIo;
use prometheus::{Encoder, HistogramVec, IntCounterVec, IntGaugeVec, TextEncoder};
use tokio::net::TcpListener;

pub const METRICS_PORT: u16 = 9889;
pub const METRICS_ENDPOINT: &str = "/metrics";

lazy_static! {
	// Metric collectors
	pub static ref MESSAGE_COUNTER: IntCounterVec = {
		let opts = opts!("message_counter", "Displays number of messages");
		register_int_counter_vec!(opts, &["chat_type", "command"]).unwrap()
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
	let metrics_addr = SocketAddr::from(([0, 0, 0, 0], METRICS_PORT));
	let listener = TcpListener::bind(metrics_addr).await?;
	loop {
		let (stream, _) = listener.accept().await?;

		// Use an adapter to access something implementing `tokio::io` traits as if they implement
		// `hyper::rt` IO traits.
		let io = TokioIo::new(stream);

		info!("Serving metrics on {metrics_addr}");

		// Spawn a tokio task to serve multiple connections concurrently
		tokio::task::spawn(async move {
			// Finally, we bind the incoming connection to our `hello` service
			if let Err(err) = http1::Builder::new()
				// `service_fn` converts our function in a `Service`
				.serve_connection(io, service_fn(metrics))
				.await
			{
				error!("Error serving connection: {:?}", err);
			}
		});
	}
}

async fn metrics(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
	match (req.method(), req.uri().path()) {
		(&Method::GET, METRICS_ENDPOINT) => {
			let encoder = TextEncoder::new();
			let metric_families = prometheus::gather();
			let mut buffer = vec![];
			encoder.encode(&metric_families, &mut buffer).unwrap();
			Ok(Response::new(Full::new(Bytes::from(buffer))))
		}
		_ => {
			let mut not_found = Response::default();
			*not_found.status_mut() = StatusCode::NOT_FOUND;
			Ok(not_found)
		}
	}
}
