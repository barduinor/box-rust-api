use std::error::Error;

// use crate::box_authentication::authorization_request;
use crate::box_authentication::authorization_request::AuthorizationGrant;
use crate::box_authentication::authorization_request::AuthorizationRequest;
use crate::box_authentication::authorization_request::AuthorizationRequestOptional;
use openapi::apis::authorization_api::post_oauth2_token;
use openapi::apis::authorization_api::PostOauth2TokenError;
use openapi::apis::authorization_api::PostOauth2TokenParams;
use openapi::models::file_all_of_shared_link::Access;
use openapi::models::AccessToken;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OAuth2 {
    pub client_id: String,
    pub client_secret: String,
}

impl OAuth2 {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
        }
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

    pub async fn authenticate(&self, code: String) -> AccessToken {
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
        let access_token = post_oauth2_token(&config, params).await;
        access_token.expect("Unable to get access token")
    }
}
