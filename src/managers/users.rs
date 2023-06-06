use crate::box_api_client::BoxApiClient;
use crate::models::UserFull;

pub async fn me(api: &BoxApiClient) -> UserFull {
    let response_str = api.get("users/me").await;
    serde_json::from_str(&response_str).unwrap()
}
