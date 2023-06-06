// use cargo run --bin users_main to run this file
// use dotenv;

use std::env;
use box_rust_sdk::authorization::DeveloperTokenAuthorizaton;
use box_rust_sdk::box_api_client::BoxApiClient;
use box_rust_sdk::managers::users;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    let developer_token = env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN not set");

    let auth = DeveloperTokenAuthorizaton::new(developer_token);
    let api = BoxApiClient::new(auth);
    let user = users::me(&api).await;

    println!("Current user:\n{:?}\n", user);
}
