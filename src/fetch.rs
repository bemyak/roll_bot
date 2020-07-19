use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

use futures::future::{join_all, try_join_all, BoxFuture, FutureExt, TryFutureExt};

use serde_json::Value as JsonValue;

use crate::collection::{Collection, COLLECTIONS};

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
            .map(|url| download(url.field_name, format!("{}/{}", BASE_URL, url.url)))
            .collect::<Vec<_>>();

        let results = try_join_all(work).await?;
        let values = results.into_iter().flatten().collect::<Vec<_>>();
        Ok(values)
    }
}

pub async fn fetch() -> Result<HashMap<&'static str, Vec<JsonValue>>, Box<dyn Error + Send + Sync>>
{
    let work = COLLECTIONS.iter().map(|item| {
        item.fetch()
            .map_ok(move |result| (item.get_default_collection(), result))
    });
    let result = try_join_all(work).await?.into_iter().collect();
    info!("Fetch complete!");
    Ok(result)
}

async fn download(
    field_name: &'static str,
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
                let items = obj.get(field_name).ok_or(FetchError {
                    url: file_url,
                    desc: format!("Response object doesn't have field {}", field_name),
                })?;
                if let JsonValue::Array(arr) = items {
                    // TODO: Do not filter out _copy elements when they can be formatted properly
                    let arr = arr
                        .to_vec()
                        .into_iter()
                        .filter(|val| {
                            if let JsonValue::Object(obj) = val {
                                if let Some(_) = obj.get("_copy") {
                                    false
                                } else {
                                    true
                                }
                            } else {
                                true
                            }
                        })
                        .collect::<Vec<JsonValue>>();

                    return Ok(arr);
                }
            }
            Ok(Vec::new())
        }
        reqwest::StatusCode::NOT_FOUND => download_indexed(field_name, url).await,
        _ => Err::<_, Box<dyn Error + Send + Sync>>(Box::new(FetchError {
            url: file_url,
            desc: format!("Unexpected status code: {}", response.status()),
        })),
    }
}

fn download_indexed(
    field_name: &'static str,
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
                .map(|file| download(field_name, url.clone() + "/" + remove_extension(file)))
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
