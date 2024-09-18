use std::env::var;

use dotenv::dotenv;

fn main() {
    // load environment variables
    dotenv().ok();
    let zotero_api_token = var("ZOTERO_API_TOKEN").expect("ZOTERO_API_TOKEN must be set.");
    let zotero_group_id = var("ZOTERO_GROUP_ID").expect("ZOTERO_GROUP_ID must be set.");
}
