use async_trait::async_trait;
use reqwest::{Client, Response, StatusCode, Url};
use serde::{Deserialize, Serialize};
use tiny_http::Server;

use crate::authorization::Authorization;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct OAuth {
    access_token: String,
    expires_in: i16,
    refresh_token: String,
}
#[allow(dead_code)]
pub struct OAuthAuthorizaton {
    client_id: String,
    client_secret: String,
    oauth: OAuth,
}

#[async_trait]
impl Authorization for OAuthAuthorizaton {
    async fn access_token(&self) -> &String {
        &self.oauth.access_token
    }

    async fn bearer_token(&self) -> String {
        format!("Bearer {}", self.access_token().await)
    }
}

impl OAuthAuthorizaton {
    pub fn new(client_id: &String, client_secret: &String, oauth: &OAuth) -> Self {
        OAuthAuthorizaton {
            client_id: String::from(client_id),
            client_secret: String::from(client_secret),
            oauth: oauth.clone(),
        }
    }
}

fn get_authorization_url(client_id: &String) -> String {
    Url::parse_with_params(
        "https://account.box.com/api/oauth2/authorize?response_type=code",
        &[("client_id", client_id)],
    )
    .unwrap()
    .to_string()
}

pub fn authorize_user(client_id: &String) -> String {
    let server = Server::http("0.0.0.0:5000").unwrap();

    println!("Auth URL {}", get_authorization_url(client_id));

    for request in server.incoming_requests() {
        println!(
            "received request! method: {:?}, url: {:?}, headers: {:?}",
            request.method(),
            request.url(),
            request.headers()
        );
        if request.url().contains("/?code=") {
            match request.url().strip_prefix("/?code=") {
                None => panic!("We should have got a code back"),
                Some(code) => {
                    println!("got the code {}", code);
                    return String::from(code);
                }
            }
        }
    }

    panic!("We should have got a code back");
}

#[derive(Debug, Serialize)]
struct OAuth2RequestToken {
    client_id: String,
    client_secret: String,
    code: String,
    grant_type: String,
}

pub async fn request_access_token(
    client_id: &String,
    client_secret: &String,
    code: &String,
) -> String {
    let url = String::from("https://api.box.com/oauth2/token");
    let form = OAuth2RequestToken {
        client_id: String::from(client_id),
        client_secret: String::from(client_secret),
        code: String::from(code),
        grant_type: String::from("authorization_code"),
    };

    let response = Client::new().post(&url).form(&form).send().await;

    let response: Response = match response {
        Ok(r) => r,
        Err(err) => {
            panic!("Request failed with {}", err)
        }
    };

    match response.status() {
        StatusCode::OK => response.text().await.unwrap(),
        StatusCode::UNAUTHORIZED => {
            panic!("Not authorized")
        }
        _ => {
            let error_code = response.status();
            match response.text().await {
                Ok(body) => {
                    panic!(
                        "Request to {} failed with code {} and body {}",
                        url.as_str(),
                        error_code,
                        body
                    );
                }
                Err(_) => {
                    panic!(
                        "Request to {} failed with code {}, body could not be retrieved",
                        url.as_str(),
                        error_code
                    );
                }
            }
        }
    }
}
