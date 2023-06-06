// use cargo run --example users_main to run this file
// use dotenv;

use box_rust_sdk::authorization::DeveloperTokenAuthorizaton;
use box_rust_sdk::box_api_client::BoxApiClient;
use box_rust_sdk::managers::users;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    let developer_token = env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN not set");

    let auth = DeveloperTokenAuthorizaton::new(developer_token);
    let api = BoxApiClient::new(auth);
    let user = users::me(&api).await;

    println!("Current user:\n{user:#?}\n");
}
