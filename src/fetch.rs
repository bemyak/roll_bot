use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

use futures::future::{join_all, try_join_all, BoxFuture, FutureExt};

use serde_json::Value as JsonValue;

use crate::collection::*;

const BASE_URL: &str = "https://5e.tools/data";
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

impl Collection {
    pub async fn fetch(&self) -> Result<Vec<JsonValue>, Box<dyn Error + Send + Sync>> {
        let work = self
            .urls
            .iter()
            .map(|url| download(format!("{}/{}", BASE_URL, url)))
            .collect::<Vec<_>>();

        let results = try_join_all(work).await?;
        let values = results.into_iter().flatten().collect::<Vec<_>>();
        Ok(values)
    }
}

pub async fn fetch() -> Result<HashMap<String, Vec<JsonValue>>, Box<dyn Error + Send + Sync>> {
    let work = COLLECTIONS.iter().map(|item| item.fetch());
    let fetch_results = try_join_all(work).await?;
    info!("Fetch complete!");
    let mut result: HashMap<String, Vec<JsonValue>> = HashMap::new();
    fetch_results.into_iter().flatten().for_each(|value| {
        if let Some(doc) = value.as_object() {
            for (collection, value) in doc {
                if let Some(arr) = value.as_array() {
                    let arr = arr.to_vec();
                    if let Some(items) = result.get_mut(collection) {
                        items.extend(arr);
                    } else {
                        result.insert(collection.clone(), arr);
                    }
                }
            }
        }
    });
    Ok(result)
}

async fn download(url: String) -> Result<Vec<JsonValue>, Box<dyn Error + Send + Sync>> {
    let file_url = url.clone() + EXTENSION;

    let response = reqwest::get(&file_url).await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            info!("Successfully get url: {}", file_url);
            let text = response.text().await?;
            let json: JsonValue = serde_json::from_str(&text)?;
            Ok(vec![json])
        }
        reqwest::StatusCode::NOT_FOUND => download_indexed(url).await,
        _ => Err::<_, Box<dyn Error + Send + Sync>>(Box::new(FetchError {
            url: file_url,
            desc: format!("Unexpected status code: {}", response.status()),
        })),
    }
}

fn download_indexed(
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
                .map(|file| download(url.clone() + "/" + remove_extension(file)))
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
