# Zotero CSV Merge

A script for merging data from a CSV file into a Zotero Library. Originally designed only to merge changes made to the Title and Extras fields, this script was written with reusability in mind and can be easily modified to include any desired fields or duplicate an existing library/collection to a new group or user.

## Installation

1. [Install rust and cargo](https://www.rust-lang.org/tools/install). Installation can be verified by running `rustup --version && cargo --version`.
2. Clone the repository.
3. Create a `.env` in the root directory.
4. Add the required environment variables to the `.env` file.
5. Run `cargo test` which will build the binary, then test environment variables and core functionality.
6. To build the optimized binary run `cargo build --release`, the compiled binary will be located in `./target/release`. For more information, see the [`cargo-build`](https://doc.rust-lang.org/cargo/commands/cargo-build.html) chapter of The Cargo Book.
7. To run the binary after building, run `./target/release/zotero-csv-merge`. To build and run the binary in one step run `cargo run --release`. For more information, see the [`cargo-run`](https://doc.rust-lang.org/cargo/commands/cargo-run.html) chapter of The Cargo Book.
8. I would strongly recommend adding `2>&1 | tee /path/to/log.txt` to either of these commands to store logs.

## How it Works

1. A CSV of a given library or collection can be exported from Zotero as a either a backup or a more convenient way to make mass changes to the data.
2. The CSV reader goes through the file at `CSV_PATH` and deserializes the data into the fields specified in `lib.rs::PatchData`. Only the fields specified in the struct will be extracted from the CSV file and subsequently patched into Zotero.
3. The Zotero client then iterates through the extracted data, up to 50 entries at a time, posting the data into the group/user library specified by `ZOTERO_GROUP_ID`.

## Environment Variables

The `.env` file that you create in the root directory of this repository must contain the following environment variables:

- `ZOTERO_API_TOKEN`: The API token used to authenticate the Zotero client. Personal API keys can be created by logging into [zotero.org](https://zotero.org) > Profile > Edit Profile > Security.
- `ZOTERO_GROUP_ID`: The group/user ID associated with the target library you wish to merge the CSV data into. User ID can be found in the same location specified above. From my experience, the easiest way to find Group ID is to send a `GET` request to `https://api.zotero.org/users/<userID>/groups`.
- `CSV_PATH`: The path to the CSV file relative to the project directory.

## `lib.rs::PatchData`

As mentioned above, only the fields included in the `lib.rs::PatchData` struct will be extracted from the CSV and merged back into Zotero. When adding fields to the struct, it's important to make sure fields can be properly (de)serialized by serde. You should run a `GET` request on an existing item in order to see how fields are named in the API. As an example, the field for Abstract is named `Abstract Note` in the CSV but is referred to by the API as `abstractNote`. That particular field would be named `abstract_note` in the struct to adhere to Rust naming conventions and would need to be annotated with `#[serde(rename(deserialize = "Abstract Note", serialize = "abstractNote"))]` in order for both extracting from the CSV and posting to Zotero to work. All field names that are one word in the CSV are already covered by the blanket `#[serde(rename_all(deserialize = "PascalCase"))]` annotation on the struct, and do not need to be individually annotated. For more information on these annotations, see [Field Attributes](https://serde.rs/field-attrs.html) section of serde docs.

It should also be noted that `key` is a required field when updating existing entries, if it is not included, new entries will be created.

## Version

### 1.0.1

All functionality added, successfully tested and used
