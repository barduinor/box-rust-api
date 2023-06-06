// use cargo run --bin files_main to run this file
// use dotenv;

use openapi::apis::configuration::Configuration;
use openapi::apis::files_api;
use openapi::apis::folders_api;
use openapi::models::items_all_of_entries_inner::RHashType::File;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    let developer_token = env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN not set");

    let config = Configuration {
        oauth_access_token: Some(developer_token),
        // TODO: Bearer token is being ignored, consider fixing
        // config.bearer_access_token = Some(developer_token);
        ..Default::default()
    };

    // List items in root folder
    let params = folders_api::GetFoldersIdItemsParams {
        folder_id: "0".to_string(),
        ..Default::default()
    };

    let items = folders_api::get_folders_id_items(&config, params).await;
    // println!("Items:\n{:?}\n", items);
    let mut file_id = "".to_string();
    for item in items.unwrap().entries.unwrap() {
        if item.r#type == File {
            // println!("\nFound a file:\n {item:#?}");
            file_id = item.id;
            break;
        }
    }

    // Get file info
    let params = openapi::apis::files_api::GetFilesIdParams {
        file_id,
        ..Default::default()
    };

    let file_info = files_api::get_files_id(&config, params).await;

    println!("File info:\n{file_info:#?}\n");
}
