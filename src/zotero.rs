use std::{mem, thread::sleep, time::Duration};

// This module handles api requests to Zotero
use reqwest::{
    header::{HeaderValue, RETRY_AFTER},
    Client, Error, Response, StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::PatchData;

// extracts the library version
#[derive(Deserialize)]
struct LibraryResponse {
    version: usize,
}

// Zotero client object
pub struct Zotero<'a> {
    api_token: &'a str,
    client: Client,
    base_url: String,
}

impl<'a> Zotero<'a> {
    // creates a Zotero object
    pub fn set_group(group_id: &'a str, api_token: &'a str) -> Zotero<'a> {
        Zotero {
            api_token,
            client: reqwest::Client::new(),
            base_url: format!("https://api.zotero.org/groups/{}", group_id),
        }
    }

    // patches up to 50 entries at once
    async fn patch(&self, data: Vec<PatchData>) -> Result<Response, Error> {
        // fetch library version
        let library_version = self.library_version().await?;

        // send the patch
        self.patch_request(library_version, &data).await
    }

    // breaks patch data into groups <= 50 and calls patch()
    pub async fn patch_all(&self, mut data: Vec<PatchData>) -> Result<(), Error> {
        while !data.is_empty() {
            let batch: Vec<PatchData> = if data.len() >= 50 {
                data.drain(..50).collect()
            } else {
                mem::take(&mut data)
            };
            // patch the batch
            self.patch(batch).await?;
        }
        Ok(())
    }

    async fn library_version(&self) -> Result<usize, Error> {
        loop {
            let res = self
                .client
                .get(&self.base_url)
                .bearer_auth(self.api_token)
                .send()
                .await?;
            match res.status() {
                StatusCode::OK => {
                    if let Some(val) = res.headers().get("Backoff") {
                        sleep(Duration::from_secs(val.to_u64()));
                        return Ok(res.json::<LibraryResponse>().await?.version);
                    } else {
                        return Ok(res.json::<LibraryResponse>().await?.version);
                    }
                }
                StatusCode::TOO_MANY_REQUESTS => {
                    if let Some(val) = res.headers().get("Retry-After") {
                        sleep(Duration::from_secs(val.to_u64()));
                        continue;
                    }
                }
                _ => continue,
            }
        }
    }

    async fn patch_request(
        &self,
        library_version: usize,
        data: &Vec<PatchData>,
    ) -> Result<Response, Error> {
        loop {
            let res = self
                .client
                .patch(&self.base_url)
                .bearer_auth(self.api_token)
                .header("If-Unmodified-Since-Version", library_version)
                .json(data)
                .send()
                .await?;
            match res.status() {
                StatusCode::OK => {
                    if let Some(val) = res.headers().get("Backoff") {
                        sleep(Duration::from_secs(val.to_u64()));
                        return Ok(res);
                    } else {
                        return Ok(res);
                    }
                }
                StatusCode::TOO_MANY_REQUESTS => {
                    if let Some(val) = res.headers().get("Retry-After") {
                        sleep(Duration::from_secs(val.to_u64()));
                        continue;
                    }
                }
                _ => continue,
            }
        }
    }
}

pub trait HeaderValueExt {
    fn to_u64(&self) -> u64;
}

impl HeaderValueExt for HeaderValue {
    fn to_u64(&self) -> u64 {
        self.to_str()
            .expect("Failed to convert HeaderValue to str")
            .parse()
            .expect("Failed to convert HeaderValue to u64")
    }
}
