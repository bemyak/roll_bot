use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::future::Future;
use std::pin::Pin;

use futures::future::join_all;
use reqwest;
use serde_json::Value as JsonValue;

const BASE_URL: &str = "https://5e.tools/data";
const SPELLS: &str = "/spells";
const ITEMS: &str = "/items";
const BESTIARY: &str = "/bestiary";
const INDEX: &str = "/index.json";
const EXTENSION: &str = ".json";

#[derive(Debug)]
pub struct FetchError {
    url: String,
    desc: String,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while fetching {}: {}", self.url, self.desc)
    }
}

impl Error for FetchError {}

pub async fn fetch() -> Result<HashMap<&'static str, Vec<JsonValue>>, Box<dyn Error>> {
    let mut result = HashMap::new();

    for (collection, url) in vec![
        ("item", BASE_URL.to_owned() + ITEMS),
        ("monster", BASE_URL.to_owned() + BESTIARY),
        ("spell", BASE_URL.to_owned() + SPELLS),
    ] {
        let items = download(url, collection).await?;
        result.insert(collection, items);
    }

    Ok(result)
}

fn download(
    url: String,
    collection: &'static str,
) -> Pin<Box<dyn Future<Output = Result<Vec<JsonValue>, Box<dyn Error>>>>> {
    Box::pin(async move {
        let file_url = url.clone() + EXTENSION;

        let response = reqwest::get(&file_url).await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                info!("Successfully get url: {}", file_url);
                let text = response.text().await?;
                let json: JsonValue = serde_json::from_str(&text)?;
                if let JsonValue::Object(obj) = json {
                    let items = obj.get(collection).ok_or(FetchError {
                        url: file_url,
                        desc: format!("Response object doesn't have field {}", collection),
                    })?;
                    if let JsonValue::Array(arr) = items {
                        return Ok(arr.to_vec());
                    }
                }
                Ok(Vec::new())
            }
            reqwest::StatusCode::NOT_FOUND => download_indexed(url, collection).await,
            _ => Err::<_, Box<dyn Error>>(Box::new(FetchError {
                url: file_url,
                desc: format!("Unexpected status code: {}", response.status()),
            })),
        }
    })
}

async fn download_indexed(
    url: String,
    collection: &'static str,
) -> Result<Vec<JsonValue>, Box<dyn Error>> {
    info!(
        "File not found: {}, trying to treat it as a directory...",
        url
    );
    let index_url = url.clone() + INDEX;
    let index: HashMap<String, String> = reqwest::get(&index_url)
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    let children = join_all(
        index
            .values()
            .map(|file| download(url.clone() + "/" + remove_extension(file), collection))
            .collect::<Vec<_>>(),
    )
    .await;
    Ok(children.into_iter().flatten().flatten().collect::<Vec<_>>())
}

fn remove_extension(s: &str) -> &str {
    match s.rfind(".json") {
        Some(i) => &s[..i],
        None => s,
    }
}
