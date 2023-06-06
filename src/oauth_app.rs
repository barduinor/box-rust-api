// oauth app.rs:
// checks to see if there are tokens saved on file
// if not, it will open a browser window to authorize the app
// if so uses stored tokens to make api calls

mod box_authentication;
mod http_request_listener;
mod utils;

use http_request_listener::request_process;
use std::env;
use std::process::exit;

use utils::save_access_token;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI not set");

    let hostname = env::var("HOSTNAME").expect("HOSTNAME not set");
    let port = env::var("PORT").expect("PORT not set");

    let oauth = box_authentication::box_oauth::OAuth2 {
        client_id,
        client_secret,
    };
    println!("\n{:?}\n", oauth);

    let (authorization_url, state) = oauth.get_authorization_url(redirect_uri);
    println!("URL:\n{:?}\n", authorization_url);
    println!("State:\n{:?}\n", state);

    if webbrowser::open(&authorization_url).is_ok() {
        println!("Opened browser successfully");
    } else {
        println!("Failed to open browser");
    }

    let hostname_port = hostname + ":" + &port;
    let server = tiny_http::Server::http(hostname_port).unwrap();
    println!("Listening on {}", server.server_addr());

    let authorize_response = match request_process(server) {
        Ok(response) => response,
        Err(e) => {
            println!("error: {}", e);
            exit(1)
        }
    };

    if authorize_response.error.is_some() {
        println!(
            "Error: {:?} {:?}",
            authorize_response.error.to_owned(),
            authorize_response.error_description.to_owned()
        );
        exit(1)
    }

    // TODO: check state
    if authorize_response.state != Some(state.clone()) {
        println!(
            "State is different than expected\n expected: {:?}\n received: {:?}\n",
            state, authorize_response.state
        );
        exit(1)
    }

    let tokens = oauth
        .authenticate(authorize_response.code.expect("xxx"))
        .await;

    println!("Tokens:\n{:?}\n", tokens);

    save_access_token(tokens);
}
