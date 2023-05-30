use dotenv;

use std::env;
use openapi::apis::configuration::Configuration;
use openapi::apis::users_api;

#[tokio::main]
async fn main(){
    dotenv::dotenv().expect("Failed to read .env file");

    let developer_token = env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN not set");

    // let mut config = Configuration::new();
    // config.bearer_access_token = Some(developer_token);
    // // config.base_path = api_url;
    // config.base_path = "https://api.box.com/2.0".to_owned();

    let mut config = Configuration::default();
    // config.bearer_access_token = Some(developer_token); // bearer_access_token is ignored
    config.oauth_access_token = Some(developer_token);
    
    println!("{:?}", config);
    
    
    // let api_client = openapi::apis::APIClient::new(config);
    let fields: Option<Vec<String>> = Some(vec!["name".to_owned(), "login".to_owned(), "id".to_owned(), "type".to_owned()]);
    // let fields = Some(vec!["".to_owned()]);

    let params = users_api::GetUsersMeParams{
        fields: fields,
    };

    // let users_api = users_api::get_users_me(&config, params);
    
    let users = users_api::get_users_me(&config, params).await;

    println!("{:?}", users);
}