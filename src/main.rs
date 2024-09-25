use std::env::var;

use dotenvy::dotenv;
use zotero_csv_merge::{csv::CsvReader, zotero::*};

#[tokio::main]
async fn main() {
    // load environment variables
    dotenv().ok();
    let zotero_api_token = var("ZOTERO_API_TOKEN").expect("ZOTERO_API_TOKEN must be set.");
    let zotero_group_id = var("ZOTERO_GROUP_ID").expect("ZOTERO_GROUP_ID must be set.");
    let csv_path = var("CSV_PATH").expect("CSV_PATH must be set.");

    // initialize zotero client and csv reader
    let zotero = Zotero::set_group(&zotero_group_id, &zotero_api_token);
    let mut reader = CsvReader::new(csv_path);
    // extract relevant data from the csv file
    let data = reader.extract().expect("Failure reading csv file");

    // patch the library
    zotero
        .patch_all(data)
        .await
        .expect("Error during the patching process");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_reader() {
        dotenv().ok();
        let csv_path = var("CSV_PATH").expect("CSV_PATH must be set.");
        let mut reader = CsvReader::new(csv_path);
        reader.extract().expect("Failure reading csv file");
    }
}
