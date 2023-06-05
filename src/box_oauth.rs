// use crate::box_utils::generate_state;
use crate::utils;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct AuthorizationGrant {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,

    state: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    error_description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthorizationRequestOptional {
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_uri: Option<String>,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // redirect_uri_enc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
}

impl AuthorizationRequestOptional {
    pub fn new(redirect_uri: Option<String>, state: Option<String>, scope: Option<String>) -> Self {
        let local_state = match state {
            Some(state) => Some(state),
            None => Some(utils::generate_state::generate_state(16)),
        };

        let local_redirect_uri = match redirect_uri {
            Some(redirect_uri) => Some(urlencoding::encode(&redirect_uri).to_string()),
            None => None,
        };

        Self {
            redirect_uri: local_redirect_uri,
            state: local_state,
            scope,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthorizationRequest {
    client_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<AuthorizationRequestOptional>,
}
impl AuthorizationRequest {
    pub fn new(client_id: String, options: Option<AuthorizationRequestOptional>) -> Self {
        Self { client_id, options }
    }
    pub fn get_authorizarion_url(self) -> String {
        let local_options = self.options.expect("Unable to unwrap options");

        let mut url = "https://api.box.com/oauth2/authorize".to_string();
        url.push_str("?client_id=");
        url.push_str(&self.client_id);

        url.push_str("&response_type=code");

        if local_options.redirect_uri.is_some() {
            url.push_str("&redirect_uri=");
            url.push_str(
                &local_options
                    .redirect_uri
                    .expect("Unable to unwrap redirect_uri"),
            );
        }

        if local_options.state.is_some() {
            url.push_str("&state=");
            url.push_str(&local_options.state.expect("Unable to unwrap state"));
        }

        if local_options.scope.is_some() {
            url.push_str("&scope=");
            url.push_str(&local_options.scope.expect("Unable to unwrap scope"));
        }

        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_authorization_request_default_optional_default() {
        let authorization_request_optional = AuthorizationRequestOptional::new(None, None, None);
        assert!(authorization_request_optional.redirect_uri.is_none());
        assert!(authorization_request_optional.state.is_some());
        assert!(authorization_request_optional.scope.is_none());
    }

    #[test]
    fn test_authorization_request_optional() {
        let local_redirect_uri = Some("https://localhost:8080".to_string());
        let local_state = Some("STATE_STATE".to_string());
        let local_scope = Some("root_readwrite".to_string());
        let local_redirect_uri_enc = Some("https%3A%2F%2Flocalhost%3A8080".to_string());

        let authorization_request_optional = AuthorizationRequestOptional::new(
            local_redirect_uri,
            local_state.clone(),
            local_scope.clone(),
        );
        assert_eq!(
            local_redirect_uri_enc,
            authorization_request_optional.redirect_uri
        );
        assert_eq!(local_state, authorization_request_optional.state);
        assert_eq!(local_scope, authorization_request_optional.scope);
    }

    #[test]
    fn test_authorization_request_default() {
        let client_id = "CLIENT_ID".to_string();
        let authorization_request = AuthorizationRequest {
            client_id: client_id.clone(),
            options: None,
        };
        assert_eq!(client_id, authorization_request.client_id);
        assert!(authorization_request.options.is_none());
    }

    #[test]
    fn test_authorization_request_full() {
        let client_id = "CLIENT_ID".to_string();
        let local_redirect_uri = Some("https://localhost:8080".to_string());
        let local_state = Some("STATE_STATE".to_string());
        let local_scope = Some("root_readwrite".to_string());
        let local_redirect_uri_enc = Some("https%3A%2F%2Flocalhost%3A8080".to_string());

        let authorization_request = AuthorizationRequest {
            client_id: client_id.clone(),
            options: Some(AuthorizationRequestOptional::new(
                local_redirect_uri,
                local_state,
                local_scope.clone(),
            )),
        };
        assert_eq!(client_id, authorization_request.client_id);
        assert!(authorization_request.options.is_some());
        let local_auth_request_options = authorization_request
            .options
            .expect("Unable to unwrap options");
        assert_eq!(
            local_redirect_uri_enc,
            local_auth_request_options.redirect_uri
        );
        assert!(local_auth_request_options.state.is_some());
        assert_eq!(local_scope, local_auth_request_options.scope);
    }

    #[test]
    fn test_get_authorization_url() {
        let client_id = "CLIENT_ID".to_string();
        let local_redirect_uri = Some("https://localhost:8080".to_string());
        let local_state = Some("STATE_STATE".to_string());
        let local_scope = Some("root_readwrite".to_string());
        // let local_redirect_uri_enc = Some("https%3A%2F%2Flocalhost%3A8080".to_string());

        let authorization_request = AuthorizationRequest {
            client_id,
            options: Some(AuthorizationRequestOptional::new(
                local_redirect_uri,
                local_state,
                local_scope,
                // local_scope.clone(),
            )),
        };

        let authorizarion_request_url = authorization_request.get_authorizarion_url();
        println!("\n{:?}\n", authorizarion_request_url);
        // https://api.box.com/oauth2/authorize
        // ?client_id=CLIENT_ID
        // &response_type=code
        // &redirect_uri=https%3A%2F%2Flocalhost%3A8080
        // &state=STATE_STATE
        // &scope=root_readwrite

        assert!(authorizarion_request_url.contains("https://api.box.com/oauth2/authorize"));
        assert!(authorizarion_request_url.contains("?client_id=CLIENT_ID&"));
        assert!(authorizarion_request_url.contains("&response_type=code&"));
        assert!(authorizarion_request_url.contains("&redirect_uri=https%3A%2F%2Flocalhost%3A8080&"));
        assert!(authorizarion_request_url.contains("&state=STATE_STATE&"));
        assert!(!authorizarion_request_url.contains("&scope=root_readwrite&"));
    }
}
