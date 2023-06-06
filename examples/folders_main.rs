// use cargo run --bin folders_main to run this file
// use dotenv;

use box_rust_sdk::authorization::DeveloperTokenAuthorizaton;
use box_rust_sdk::box_api_client::BoxApiClient;
use box_rust_sdk::managers::folders;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    let developer_token = env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN not set");
    let auth = DeveloperTokenAuthorizaton::new(developer_token);
    let api = BoxApiClient::new(auth);

    let items = folders::items(&api, &String::from("0")).await;

    for item in items.entries.unwrap() {
        println!(
            "\nItem: type {:?} name: {}",
            item.item_type,
            item.name.unwrap()
        );
    }
}
