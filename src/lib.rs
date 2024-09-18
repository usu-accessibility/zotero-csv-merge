use serde::Serialize;

pub mod zotero;

// the data to be sent in a patch request
#[derive(Serialize)]
struct PatchData {
    key: String,
    title: String,
    extra: String,
}
