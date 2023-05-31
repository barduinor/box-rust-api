// use cargo run --bin files_main to run this file
use dotenv;

use std::env;
use openapi::apis::configuration::Configuration;
use openapi::apis::folders_api;
use openapi::models::items_all_of_entries_inner::RHashType::File;
use openapi::apis::files_api;



#[tokio::main]
async fn main(){
    dotenv::dotenv().expect("Failed to read .env file");

    let developer_token = env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN not set");

    let mut config = Configuration::default();
    // TODO: Bearer token is being ignored, consider fixing
    // config.bearer_access_token = Some(developer_token);
    config.oauth_access_token = Some(developer_token);
    // println!("Configuration:\n{:?}\n", config);

    // List items in root folder
    let mut params = folders_api::GetFoldersIdItemsParams::default();
    params.folder_id = "0".to_string();

    let items = folders_api::get_folders_id_items(
        &config, 
        params)
        .await;
    // println!("Items:\n{:?}\n", items);
    let mut file_id = "".to_string();
    for item in items.unwrap().entries.unwrap() {
        if item.r#type == File {
            println!("\nFound a file: {:?}",item);
            file_id = item.id;
            break;
        }

    }

    // Get file info
    let mut params = files_api::GetFilesIdParams::default();
    params.file_id = file_id;

    let file_info = files_api::get_files_id(
        &config, 
        params)
        .await;

    println!("File info:\n{:?}\n", file_info);
    

    


}