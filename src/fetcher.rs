extern crate hyper_tls;

use fetcher::hyper_tls::HttpsConnector;
use futures::{future, sync::mpsc::Sender, Future, Sink, Stream};
use hyper::{client::HttpConnector, Body, Chunk, Client, Error, Response, StatusCode, Uri};
use serde_json::{self, Value};
use tokio::runtime::TaskExecutor;

const BASE_URL: &str = "https://5e.tools/data";
const SPELLS: &str = "/spells";
const ITEMS: &str = "/items";
const BESTIARY: &str = "/bestiary";
const INDEX: &str = "/index.json";
const EXTENSION: &str = ".json";

pub fn fetch(executor: &TaskExecutor, tx: Sender<Value>) {
    vec![SPELLS, ITEMS, BESTIARY]
        .iter()
        .map(|url| get_work(tx.clone(), url))
        .for_each(|work| executor.spawn(work))
}

fn get_work(tx: Sender<Value>, url: &'static str) -> impl Future<Item = (), Error = ()> {
    let uri = BASE_URL.to_string() + url + EXTENSION;
    debug!("Fetching: {}", uri);
    let uri: Uri = uri.parse().unwrap();
    return send_get_request(uri)
        .from_err::<FetchError>()
        .and_then(move |resp| process_response(url, resp))
        .map(move |chunk| {
            debug!("Parsing...");
            let value: Value = serde_json::from_slice(&chunk).unwrap();
            debug!("Sending...");
            let result = tx.send(value).wait();
            match result {
                Ok(_) => info!("Ok"),
                Err(err) => error!("Fuck {}", err),
            }
            debug!("Parsed!");
        }).map_err(|err| match err {
            FetchError::UnexpectedStatusCode(code) => {
                warn!("Unexpected status code occured: {}", code)
            }
            FetchError::HyperError(error) => error!("Hyper error: {}", error),
            FetchError::NoError => error!("How did we get here?!"),
        });
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
) -> impl Future<Item = Chunk, Error = FetchError> {
    debug!("Processing response...");
    let result = match resp.status() {
        StatusCode::OK => unwrap_response(resp),
        StatusCode::NOT_FOUND => process_not_found(url),
        _ => get_status_code_error(resp.status()),
    };
    return result;
}

fn get_client() -> Client<HttpsConnector<HttpConnector>> {
    debug!("Getting client...");
    Client::builder().build(HttpsConnector::new(4).unwrap())
}

fn process_not_found(url: &str) -> Box<Future<Item = Chunk, Error = FetchError> + Send> {
    let uri = BASE_URL.to_string() + url + INDEX;
    debug!("Got 404! Trying url: {}", uri);
    let uri = uri.parse().unwrap();
    let result = send_get_request(uri)
        .from_err::<FetchError>()
        .and_then(|resp| match resp.status() {
            StatusCode::OK => unwrap_response(resp),
            _ => get_status_code_error(resp.status()),
        });
    return Box::new(result);
}

fn unwrap_response(resp: Response<Body>) -> Box<Future<Item = Chunk, Error = FetchError> + Send> {
    debug!("Unwrapping response...");
    let future = resp.into_body().concat2().from_err::<FetchError>();
    Box::new(future)
}

fn get_status_code_error(
    status_code: StatusCode,
) -> Box<Future<Item = Chunk, Error = FetchError> + Send> {
    return Box::new(future::err::<Chunk, FetchError>(
        FetchError::UnexpectedStatusCode(status_code),
    ));
}

enum FetchError {
    UnexpectedStatusCode(StatusCode),
    HyperError(Error),
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
