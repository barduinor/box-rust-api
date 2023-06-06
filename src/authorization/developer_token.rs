use async_trait::async_trait;

use crate::authorization::Authorization;

pub struct DeveloperTokenAuthorizaton {
    token: String,
}

#[async_trait]
impl Authorization for DeveloperTokenAuthorizaton {
    async fn access_token(&self) -> &String {
        &self.token
    }

    async fn bearer_token(&self) -> String {
        format!("Bearer {}", self.access_token().await)
    }
}

impl DeveloperTokenAuthorizaton {
    pub fn new(token: String) -> Self {
        DeveloperTokenAuthorizaton {
            token
        }
    }
}
