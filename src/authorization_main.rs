// use cargo run --bin authorization_main to run this file
use std::env;

// use dotenv;
// use webbrowser;

use openapi::apis::authorization_api::get_authorizarion_url;
use openapi::apis::authorization_api::GetAuthorizeParams;
// use openapi::apis::configuration::Configuration;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    // let developer_token = env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN not set");
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI not set");

    // let config = Configuration {
    //     oauth_access_token: Some(developer_token),
    //     // TODO: Bearer token is being ignored, consider fixing
    //     // config.bearer_access_token = Some(developer_token);
    //     ..Default::default()
    // };
    // println!("{:?}", config);

    let params = GetAuthorizeParams {
        client_id,
        redirect_uri: Some(redirect_uri),
        // state: Some("STATE_STATE".to_string()),
        // scope: Some("root_readwrite".to_string()),
        ..Default::default()
    };

    println!("\nParams: {:?}\n", params);

    let authorization_url = get_authorizarion_url(params);
    println!("\nAuthorization URI: {:?}\n", &authorization_url);

    if webbrowser::open(&authorization_url).is_ok() {
        println!("Opened browser successfully");
    } else {
        println!("Failed to open browser");
    }
}
