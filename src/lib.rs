use serde::{Deserialize, Serialize};

pub mod csv;
pub mod zotero;

// the data to be sent in a patch request
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PatchData {
    key: String,
    title: String,
    extra: String,
}
