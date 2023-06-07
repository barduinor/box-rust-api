use crate::box_api_client::BoxApiClient;

pub async fn delete(api: &BoxApiClient, folder_id: &String) {
    let url = format!("files/{}", folder_id);
    api.delete(url.as_str()).await
}
