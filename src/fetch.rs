use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

use futures::future::{join_all, BoxFuture, FutureExt};
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

pub async fn fetch() -> Result<HashMap<&'static str, Vec<JsonValue>>, Box<dyn Error + Send + Sync>>
{
    let mut result = HashMap::new();

    for (collection, url) in vec![
        ("item", BASE_URL.to_owned() + ITEMS),
        ("monster", BASE_URL.to_owned() + BESTIARY),
        ("spell", BASE_URL.to_owned() + SPELLS),
    ] {
        result.insert(collection, download(collection, url).await?);
    }

    info!("Fetch complete!");

    Ok(result)
}

async fn download(
    collection: &'static str,
    url: String,
) -> Result<Vec<JsonValue>, Box<dyn Error + Send + Sync>> {
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
        reqwest::StatusCode::NOT_FOUND => download_indexed(collection, url).await,
        _ => Err::<_, Box<dyn Error + Send + Sync>>(Box::new(FetchError {
            url: file_url,
            desc: format!("Unexpected status code: {}", response.status()),
        })),
    }
}

fn download_indexed(
    collection: &'static str,
    url: String,
) -> BoxFuture<'static, Result<Vec<JsonValue>, Box<dyn Error + Send + Sync>>> {
    async move {
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
                .map(|file| download(collection, url.clone() + "/" + remove_extension(file)))
                .collect::<Vec<_>>(),
        )
        .await;
        Ok(children.into_iter().flatten().flatten().collect::<Vec<_>>())
    }
    .boxed()
}

fn remove_extension(s: &str) -> &str {
    match s.rfind(".json") {
        Some(i) => &s[..i],
        None => s,
    }
}
