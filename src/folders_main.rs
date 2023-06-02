// use cargo run --bin folders_main to run this file
// use dotenv;

use openapi::apis::configuration::Configuration;
use openapi::apis::folders_api;
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
    // println!("Configuration:\n{:?}\n", config);

    // Get folder information
    let params = openapi::apis::folders_api::GetFoldersIdParams {
        folder_id: "0".to_string(),
        // folder_id: "209408240392".to_string(),
        // fields: Some(vec!["id".to_owned(), "name".to_owned(), "tags".to_owned()]),
        ..Default::default()
    };

    let folder_info = folders_api::get_folders_id(&config, params).await;
    println!("\nFolder Info:\n{:?}\n", folder_info);

    // List items in folder
    let params = openapi::apis::folders_api::GetFoldersIdItemsParams {
        folder_id: "0".to_string(),
        // folder_id: "209408240392".to_string(),
        ..Default::default()
    };

    let items = folders_api::get_folders_id_items(&config, params).await;
    // println!("Items:\n{:?}\n", items);

    for item in items.unwrap().entries.unwrap() {
        println!("\nItem: {:?}", item);
    }
}
