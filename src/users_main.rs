// use cargo run --bin users_main to run this file
// use dotenv;

use openapi::apis::configuration::Configuration;
use openapi::apis::users_api;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    let developer_token = env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN not set");

    let config = Configuration {
        oauth_access_token: Some(developer_token),
        // TODO: Bearer token is being ignored, consider fixing
        // config.bearer_access_token: Some(developer_token);
        ..Default::default()
    };
    // println!("{:?}", config);

    // #[warn(clippy::needless_update)]
    let params = openapi::apis::users_api::GetUsersMeParams {
        // fields: Some(vec!["id".to_owned(), "name".to_owned(), "login".to_owned()]),
        ..Default::default()
    };

    let user_me = users_api::get_users_me(&config, params).await;
    println!("Current user:\n{:?}\n", user_me);
}
