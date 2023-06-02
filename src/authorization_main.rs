// use cargo run --bin authorization_main to run this file
use std::env;
use std::process::exit;

// use dotenv;
// use webbrowser;

use tiny_http::Request;
use tiny_http::Response;
use url::Url;

use openapi::apis::authorization_api::get_authorizarion_url;
use openapi::apis::authorization_api::GetAuthorizeParams;

// use openapi::apis::configuration::Configuration;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct UrlParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,

    state: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    error_description: Option<String>,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    // let developer_token = env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN not set");
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI not set");

    let hostname = env::var("HOSTNAME").expect("HOSTNAME not set");
    let port = env::var("PORT").expect("PORT not set");

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
    let hostname_port = hostname + ":" + &port;
    let server = tiny_http::Server::http(hostname_port).unwrap();
    println!("Listening on {}", server.server_addr());

    match server.recv() {
        Ok(rq) => {
            println!(
                "received request:\nmethod: {:?}\nurl: {:?}\nremote_addr: {:?} ",
                rq.method(),
                rq.url(),
                rq.remote_addr(),
                // rq.headers()
            );

            let base_url = match Url::parse("http://127.0.0.1") {
                Ok(url) => url,
                Err(e) => {
                    println!("error: {}", e);
                    exit(1)
                }
            };

            let url = match base_url.join(rq.url()) {
                Ok(url) => url,
                Err(e) => {
                    println!("error: {}", e);
                    exit(1)
                }
            };

            let query_params: UrlParams = match serde_qs::from_str(url.query().unwrap()) {
                Ok(query_params) => query_params,
                Err(e) => {
                    println!("error: {}", e);
                    exit(1)
                }
            };

            if query_params.error.is_some() {
                println!("error: {}", query_params.error_description.unwrap());
                exit(1)
            }

            let response = Response::empty(200);
            rq.respond(response);
        }
        Err(e) => {
            println!("error: {}", e);
            exit(1)
        }
    }
}
