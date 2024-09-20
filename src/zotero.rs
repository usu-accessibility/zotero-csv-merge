use std::{mem, thread::sleep, time::Duration};

// This module handles api requests to Zotero
use reqwest::{header::HeaderValue, Client, Error, Response, StatusCode};
use serde::Deserialize;

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
        println!("Fetching library version...");
        let library_version = self.library_version().await?;
        println!("Library version: {}", library_version);

        // send the patch
        println!("Sending patch for keys: {:?}", data.iter().map(|e| &e.key));
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
            let res = self.patch(batch).await?;
            println!("{:?}", res.status());
        }
        println!("All data patched.");
        Ok(())
    }

    // fetches the library version
    async fn library_version(&self) -> Result<usize, Error> {
        loop {
            // send the request and wait for response
            let res = self
                .client
                .get(&self.base_url)
                .bearer_auth(self.api_token)
                .send()
                .await?;
            // handle Backoff headers and 429 responses, resends on any response other than OK
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
            // send the request and await response
            let res = self
                .client
                .patch(&self.base_url)
                .bearer_auth(self.api_token)
                .header("If-Unmodified-Since-Version", library_version)
                .json(data)
                .send()
                .await?;
            // handle Backoff header and 429 responses, panics upon any response other than No Content
            match res.status() {
                StatusCode::NO_CONTENT => {
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
                _ => panic!("{}", res.status()),
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
