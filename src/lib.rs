use serde::{Deserialize, Serialize};

pub mod csv;
pub mod zotero;

// the data to be sent in a patch request
// the fields below are the only fields that will be extracted from the csv file
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PatchData {
    key: String,
    title: String,
    extra: String,
}
