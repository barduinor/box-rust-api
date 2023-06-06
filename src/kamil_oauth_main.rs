use std::{env};

use box_rust_sdk::authorization;
use box_rust_sdk::authorization::{OAuth, OAuthAuthorizaton};
use box_rust_sdk::box_api_client::BoxApiClient;
use box_rust_sdk::managers::users;

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenv::dotenv().expect("Failed to read .env file");
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");

    // get URL that will prompt for OAUTH authorization this also starts webserver that listein for response
    // the redirect URL is hardcoded to http://localhost:5000
    let code = authorization::authorize_user(&client_id);

    // this will do a API call and exchange code for actualt tokens
    // this should not return string but OAuth object
    let access_token_str = authorization::request_access_token(&client_id, &client_secret, &code).await;
    let oauth: OAuth = serde_json::from_str(&access_token_str).unwrap();
    println!("{:?}", oauth);

    // with OAuth we can finally create Authorization object and use it in client
    let auth = OAuthAuthorizaton::new(&client_id, &client_secret, &oauth);
    let api = BoxApiClient::new(auth);
    let response = users::me(&api).await;
    println!("{:?}", &response.unwrap());

    Ok(())
}
