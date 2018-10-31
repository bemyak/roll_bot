extern crate hyper_tls;

use fetcher::hyper_tls::HttpsConnector;
use futures::{future, sync::mpsc::Sender, Future, Sink, Stream};
use hyper::{client::HttpConnector, Body, Client, Error, Response, StatusCode, Uri};
use serde_json::{self, Value};
use tokio::runtime::TaskExecutor;

const BASE_URL: &str = "https://5e.tools/data";
const SPELLS: &str = "/spells";
const ITEMS: &str = "/items";
const BESTIARY: &str = "/bestiary";
const INDEX: &str = "/index.json";
const EXTENSION: &str = ".json";

pub struct Fetcher<'a> {
    executor: &'a TaskExecutor,
    tx: Sender<Value>,
    client: Client<HttpsConnector<HttpConnector>>,
}

impl<'a> Fetcher<'a> {
    pub fn init(executor: &'a TaskExecutor, tx: Sender<Value>) -> Self {
        Self {
            executor: executor,
            tx: tx,
            client: get_client(),
        }
    }

    pub fn fetch(&self) {
        vec![SPELLS, ITEMS, BESTIARY]
            .iter()
            .map(|url| get_work(self.tx.clone(), url))
            .for_each(|work| self.executor.spawn(work))
    }
}

fn get_work(tx: Sender<Value>, url: &'static str) -> impl Future<Item = (), Error = ()> {
    let uri = BASE_URL.to_string() + url + EXTENSION;
    debug!("Fetching: {}", uri);
    let uri: Uri = uri.parse().unwrap();
    return send_get_request(uri)
        .from_err::<FetchError>()
        .and_then(move |resp| process_response(url, resp))
        .map(move |value| match value {
            FetchResult::SingleResult(value) => save_value_to_db(tx, value),
            FetchResult::MultiResult(mut values) => values
                .drain(..)
                .for_each(|value| save_value_to_db(tx.clone(), value)),
        }).map_err(|err| match err {
            FetchError::UnexpectedStatusCode(code) => {
                warn!("Unexpected status code occured: {}", code)
            }
            FetchError::HyperError(error) => error!("Hyper error: {}", error),
            FetchError::NoError => error!("How did we get here?!"),
            FetchError::BadJson => error!("Problem occured while parsing json"),
        });
}

fn save_value_to_db(tx: Sender<Value>, value: Value) {
    debug!("Saving...");
    let result = tx.send(value).wait();
    match result {
        Ok(_) => info!("Ok"),
        Err(err) => error!("Fuck {}", err),
    };
}

fn send_get_request(uri: Uri) -> impl Future<Item = Response<Body>, Error = ()> {
    let client = get_client();
    debug!("Making http response...");
    client
        .get(uri)
        .map(|res| res)
        .map_err(move |err| warn!("Cannot get cause of {}", err))
}

fn process_response(
    url: &str,
    resp: Response<Body>,
) -> impl Future<Item = FetchResult, Error = FetchError> {
    debug!("Processing response...");
    match resp.status() {
        StatusCode::OK => process_ok(resp),
        // StatusCode::NOT_FOUND => process_not_found(url),
        _ => get_status_code_error(resp.status()),
    }
}

fn get_client() -> Client<HttpsConnector<HttpConnector>> {
    debug!("Getting client...");
    Client::builder().build(HttpsConnector::new(4).unwrap())
}

fn process_ok(resp: Response<Body>) -> Box<Future<Item = FetchResult, Error = FetchError> + Send> {
    let future = unwrap_response(resp).map(|value| FetchResult::SingleResult(value));
    Box::new(future)
}

fn process_not_found(url: &str) -> Box<Future<Item = FetchResult, Error = FetchError> + Send> {
    let uri = BASE_URL.to_string() + url + INDEX;
    debug!("Got 404! Trying url: {}", uri);
    let uri = uri.parse().unwrap();
    let result = send_get_request(uri)
        .from_err::<FetchError>()
        .and_then(|resp| match resp.status() {
            StatusCode::OK => process_ok(resp),
            _ => get_status_code_error(resp.status()),
        }).and_then(|result| match result {
            FetchResult::SingleResult(value) => Ok(value),
            _ => Err(FetchError::BadJson),
        }).and_then(|value| match value {
            Value::Object(map) => Ok(map),
            _ => Err(FetchError::BadJson),
        }).and_then(|map| {
            let mut parts: Vec<String> = Vec::new();
            map.values().for_each(|value| {
                if let Value::String(string) = value {
                    parts.push(string.to_string());
                }
            });
            if parts.is_empty() {
                Err(FetchError::BadJson)
            } else {
                Ok(parts)
            }
        }).map(|parts| {
            let result: Vec<Value> = Vec::new();
            // parts.iter().map(|part| {
            //     let uri = BASE_URL.to_string() + url + part;
            //     debug!("Fetching: {}", uri);
            //     let uri: Uri = uri.parse().unwrap();
            //     let work = send_get_request(uri).and_then(|response| process_ok(response));
            //     get_client().
            // });
            FetchResult::MultiResult(result)
        });
    return Box::new(result);
}

fn unwrap_response(resp: Response<Body>) -> impl Future<Item = Value, Error = FetchError> {
    debug!("Unwrapping response...");
    resp.into_body()
        .concat2()
        .from_err::<FetchError>()
        .map(|chunk| serde_json::from_slice(&chunk).unwrap())
}

fn get_status_code_error(
    status_code: StatusCode,
) -> Box<Future<Item = FetchResult, Error = FetchError> + Send> {
    return Box::new(future::err::<FetchResult, FetchError>(
        FetchError::UnexpectedStatusCode(status_code),
    ));
}

enum FetchResult {
    SingleResult(Value),
    MultiResult(Vec<Value>),
}

enum FetchError {
    UnexpectedStatusCode(StatusCode),
    HyperError(Error),
    BadJson,
    NoError,
}

impl From<()> for FetchError {
    fn from(_: ()) -> FetchError {
        FetchError::NoError
    }
}

impl From<Error> for FetchError {
    fn from(hyper_error: Error) -> FetchError {
        FetchError::HyperError(hyper_error)
    }
}
