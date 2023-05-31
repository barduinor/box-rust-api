// use cargo run --bin folders_main to run this file
use dotenv;

use std::env;
use openapi::apis::configuration::Configuration;
use openapi::apis::folders_api;


#[tokio::main]
async fn main(){
    dotenv::dotenv().expect("Failed to read .env file");

    let developer_token = env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN not set");

    let mut config = Configuration::default();
    // TODO: Bearer token is being ignored, consider fixing
    // config.bearer_access_token = Some(developer_token);
    config.oauth_access_token = Some(developer_token);
    println!("{:?}", config);

}