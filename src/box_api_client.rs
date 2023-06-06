use std::str::FromStr;

use reqwest::{Client, Response, Error, StatusCode, Url};
use reqwest::multipart::Form;
use serde::Serialize;

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

    pub async fn get(&self, path: &str) -> Option<String> {
        let url = self.url(path);
        let result = self.client.get(url.clone())
            .header("Authorization", self.authorization.bearer_token().await)
            .send()
            .await;
        self.response_to_string(result, &url).await
    }

    pub async fn post(&self, path: &str, body: &impl Serialize) -> String {
        let url = self.url(path);
        let result = self.client.post(url.clone())
            .json(body)
            .header("Authorization", self.authorization.bearer_token().await)
            .send()
            .await;
        self.response_to_string(result, &url).await.unwrap()
    }

    pub async fn delete(&self, path: &str) -> () {
        let url = self.url(path);
        let response = self.client.delete(url.clone())
            .header("Authorization", self.authorization.bearer_token().await)
            .send()
            .await;
        self.response_to_string(response, &url).await;
    }

    pub async fn multipart(&self, form: Form) -> Option<String> {
        let url = self.url("https://upload.box.com/api/2.0/files/content");
        let response = self.client.post(url.clone())
            .header("Authorization", self.authorization.bearer_token().await)
            .multipart(form)
            .send()
            .await;
        self.response_to_string(response, &url).await
    }

    fn url(&self, path: &str) -> Url{
        Url::from_str(self.base_api_url.as_str()).unwrap().join(path).unwrap()
    }

    async fn response_to_string(&self, response: Result<Response, Error>, url: &Url) -> Option<String> {
        let result = match response {
            Ok(r) => { r }
            Err(err) => {
                panic!("Request failed with {}", err)
            }
        };

        match result.status() {
            StatusCode::OK => {
                Some(result.text().await.unwrap())
            }
            StatusCode::CREATED => {
                Some(result.text().await.unwrap())
            }
            StatusCode::NO_CONTENT => {
                None
            }
            StatusCode::UNAUTHORIZED => {
                panic!("Not authorized")
            }
            StatusCode::NOT_FOUND => {
                None
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
