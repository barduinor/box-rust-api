use crate::box_authentication::authorization_request::AuthorizationRequest;
use crate::box_authentication::authorization_request::AuthorizationRequestOptional;
use openapi::apis::authorization_api::post_oauth2_token;

use openapi::apis::authorization_api::PostOauth2TokenParams;

use openapi::models::AccessToken;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct OAuth2 {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: String,
}

impl OAuth2 {
    #[allow(dead_code)]
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
            access_token: String::new(),
        }
    }
    #[allow(dead_code)]
    pub fn access_token(&self) -> String {
        self.access_token.clone()
    }

    pub fn get_authorization_url(&self, redirect_uri: String) -> (String, String) {
        let options = AuthorizationRequestOptional::new(Some(redirect_uri), None, None);
        let authorization_request = AuthorizationRequest {
            client_id: self.client_id.clone(),
            options: Some(options),
        };
        let state = authorization_request
            .options
            .clone()
            .unwrap()
            .state
            .unwrap();

        (authorization_request.get_authorizarion_url(), state)
    }

    pub async fn authenticate(mut self, code: String) -> AccessToken {
        // let config = openapi::apis::configuration::Configuration::default();

        let config = openapi::apis::configuration::Configuration {
            base_path: "https://api.box.com".to_string(),
            ..Default::default()
        };

        let params = PostOauth2TokenParams {
            grant_type: "authorization_code".to_string(),
            code: Some(code),
            client_id: Some(self.client_id.clone()),
            client_secret: Some(self.client_secret.clone()),

            ..Default::default()
        };
        let access_token = match post_oauth2_token(&config, params).await {
            Ok(access_token) => access_token,
            Err(e) => {
                println!("Error: {:?}", e);
                AccessToken::default()
            }
        };
        self.access_token = access_token
            .clone()
            .access_token
            .expect("Error unwrapping access token");
        access_token
    }
}
