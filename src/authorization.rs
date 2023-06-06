use async_trait::async_trait;

#[async_trait]
pub trait Authorization {
    async fn access_token(&self) -> &String;
    async fn bearer_token(&self) -> String;
}

pub use self::developer_token::DeveloperTokenAuthorizaton;
pub use self::outh_authorization::OAuthAuthorizaton;
pub use self::outh_authorization::OAuth;
pub use self::outh_authorization::authorize_user;
pub use self::outh_authorization::request_access_token;
mod developer_token;
mod outh_authorization;
