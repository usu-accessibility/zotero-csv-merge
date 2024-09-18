// This module handles api requests to Zotero
use reqwest::{Client, Error, Response};
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
        let library_version = self
            .client
            .get(&self.base_url)
            .bearer_auth(self.api_token)
            .send()
            .await?
            .json::<LibraryResponse>()
            .await?
            .version;

        // send the patch
        self.client
            .patch(&self.base_url)
            .bearer_auth(self.api_token)
            .header("If-Unmodified-Since-Version", library_version)
            .json(&data)
            .send()
            .await
    }

    // breaks patch data into groups <= 50 and calls patch()
    pub fn patch_all(&self) {
        todo!()
    }
}
