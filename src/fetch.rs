use std::collections::HashMap;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;

use ejdb::bson;
use futures::future::join_all;
use reqwest;
use tokio::try_join;

use crate::db::DndDatabase;

const BASE_URL: &str = "https://5e.tools/data";
const SPELLS: &str = "/spells";
const ITEMS: &str = "/items";
const BESTIARY: &str = "/bestiary";
const INDEX: &str = "/index.json";
const EXTENSION: &str = ".json";

pub async fn fetch(db: DndDatabase) -> Result<(), Box<dyn Error>> {
    try_join!(
        download(BASE_URL.to_owned() + SPELLS, "spell", db.clone()),
        download(BASE_URL.to_owned() + ITEMS, "item", db.clone()),
        download(BASE_URL.to_owned() + BESTIARY, "monster", db.clone()),
    )?;
    Ok(())
}

fn download(
    url: String,
    collection: &'static str,
    db: DndDatabase,
) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>>>> {
    Box::pin(async move {
        let file_url = url.clone() + EXTENSION;

        let response = reqwest::get(&file_url).await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                info!("Successfully get url: {}", file_url);
                let text = response.text().await?;
                let json: serde_json::Value = serde_json::from_str(&text)?;
                db.save_collection(json, collection)?;
                Ok(())
            }
            reqwest::StatusCode::NOT_FOUND => download_indexed(url, collection, db.clone()).await,
            _ => Err::<(), Box<dyn Error>>(Box::new(bson::DecoderError::Unknown(format!(
                "Bad HTTP Status while getting {}: {}",
                file_url,
                response.status()
            )))),
        }
    })
}

async fn download_indexed(
    url: String,
    collection: &'static str,
    db: DndDatabase,
) -> Result<(), Box<dyn Error>> {
    info!(
        "File not found: {}, trying to treat it as a directory...",
        url
    );
    let index_url = url.clone() + INDEX;
    let index: HashMap<String, String> = reqwest::get(&index_url)
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    join_all(
        index
            .values()
            .map(|file| {
                download(
                    url.clone() + "/" + remove_extension(file),
                    collection,
                    db.clone(),
                )
            })
            .collect::<Vec<_>>(),
    )
    .await;
    Ok(())
}

fn remove_extension(s: &str) -> &str {
    match s.rfind(".json") {
        Some(i) => &s[..i],
        None => s,
    }
}
