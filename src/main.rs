use std::env::var;

use dotenvy::dotenv;
use zotero_csv_merge::zotero::*;

#[tokio::main]
async fn main() {
    // load environment variables
    dotenv().ok();
    let zotero_api_token = var("ZOTERO_API_TOKEN").expect("ZOTERO_API_TOKEN must be set.");
    let zotero_group_id = var("ZOTERO_GROUP_ID").expect("ZOTERO_GROUP_ID must be set.");
    let mut zotero = Zotero::set_group(&zotero_group_id, &zotero_api_token);
}
