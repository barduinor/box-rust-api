use std::str::FromStr;

use reqwest::{Client, StatusCode, Url};

use crate::authorization::Authorization;

pub struct BoxApiClient {
    base_api_url: String,
    authorization: Box<dyn Authorization + 'static>,
    client: Client,
}

impl BoxApiClient {
    pub fn new(authorization: impl Authorization + 'static) -> Self {
        Self {
            base_api_url: String::from("https://api.box.com/2.0/"),
            authorization: Box::new(authorization),
            client: Client::new(),
        }
    }

    pub async fn get(&self, path: &str) -> String {
        let url = Url::from_str(self.base_api_url.as_str()).unwrap().join(path).unwrap();
        let result = self.client.get(url.as_str())
            .header("Authorization", self.authorization.bearer_token().await)
            .send()
            .await;

        let result = match result {
            Ok(r) => { r }
            Err(err) => {
                panic!("Request failed with {}", err)
            }
        };

        match result.status() {
            StatusCode::OK => {
                result.text().await.unwrap()
            }
            StatusCode::UNAUTHORIZED => {
                panic!("Not authorized")
            }
            _ => {
                let error_code = result.status();
                match result.text().await {
                    Ok(body) => {
                        panic!("Request to {} failed with code {} and body {}", url.as_str(), error_code, body);
                    }
                    Err(_) => {
                        panic!("Request to {} failed with code {}, body could not be retrieved", url.as_str(), error_code);
                    }
                }
            }
        }
    }
}
