use std::{mem, thread::sleep, time::Duration};

// This module handles api requests to Zotero
use reqwest::{header::HeaderValue, Client, Error, Response, StatusCode};

use crate::PatchData;

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
        println!(
            "Sending patch for keys: {:#?}",
            data.iter().map(|e| &e.key).collect::<Vec<&String>>()
        );
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
                .get(format!("{}/{}", &self.base_url, "collections"))
                .bearer_auth(self.api_token)
                .send()
                .await?;
            // handle Backoff headers and 429 responses, resends on any response other than OK
            match res.status() {
                StatusCode::OK => {
                    if let Some(val) = res.headers().get("Backoff") {
                        sleep(Duration::from_secs(val.to_u64()));
                    }
                    return Ok(res
                        .headers()
                        .get("Last-Modified-Version")
                        .expect("Version header not found")
                        .to_u64() as usize);
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

    // sends the request for a patch of up to 50 items
    async fn patch_request(
        &self,
        library_version: usize,
        data: &Vec<PatchData>,
    ) -> Result<Response, Error> {
        let mut library_version = library_version;
        loop {
            // send the request and await response
            let res = self
                .client
                .post(format!("{}/{}", &self.base_url, "items"))
                .bearer_auth(self.api_token)
                .header("If-Unmodified-Since-Version", library_version)
                .json(data)
                .send()
                .await?;
            // handle Backoff header and 429 responses, panics upon any response other than OK
            match res.status() {
                // successful
                StatusCode::OK => {
                    // checks for Backoff header and waits the specified duration
                    if let Some(val) = res.headers().get("Backoff") {
                        let seconds = val.to_u64();
                        println!("Backoff: {} seconds", seconds);
                        sleep(Duration::from_secs(seconds));
                    }
                    return Ok(res);
                }
                // rate limit exceeded
                StatusCode::TOO_MANY_REQUESTS => {
                    // extract Retry-After header, wait specified duration and resend
                    if let Some(val) = res.headers().get("Retry-After") {
                        sleep(Duration::from_secs(val.to_u64()));
                        continue;
                    }
                }
                // library version no longer matches, fetch current library version and resend
                StatusCode::PRECONDITION_FAILED => {
                    library_version = self.library_version().await?;
                    continue;
                }
                // Any other code will panic the thread
                // 409: Conflict is the only other code listed in the docs, which would indicate a bad API key
                _ => panic!("{}", res.status()),
            }
        }
    }
}
// trait enxtension to allow conversion between HeaderValue and u64
// used to extract the value from Backoff and Retry-After headers
pub trait HeaderValueExt {
    fn to_u64(&self) -> u64;
}

impl HeaderValueExt for HeaderValue {
    // function for converting between HeaderValue and u64
    fn to_u64(&self) -> u64 {
        self.to_str()
            .expect("Failed to convert HeaderValue to str")
            .parse()
            .expect("Failed to convert HeaderValue to u64")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::{dotenv, var};

    // test for the HeaderValueExt::to_u64 function
    #[tokio::test]
    async fn test_library_version() {
        // fetch relevant environment variables
        dotenv().ok();
        let api_token = var("ZOTERO_API_TOKEN").expect("ZOTERO_API_TOKEN must be set.");
        let group_id = var("ZOTERO_GROUP_ID").expect("ZOTERO_GROUP_ID must be set.");

        let zotero = Zotero::set_group(&group_id, &api_token);
        let library_version = zotero.library_version().await.unwrap();
        assert_eq!(library_version, 70117);
    }
}
