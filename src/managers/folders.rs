use crate::box_api_client::BoxApiClient;
use crate::models::FolderAllOfItemCollection;

pub async fn items(api: &BoxApiClient, folder_id: &String) -> FolderAllOfItemCollection {
    let response_string = api.get(format!("folders/{}/items", folder_id).as_str()).await;
    let items: FolderAllOfItemCollection = serde_json::from_str(&response_string).unwrap();
    items
}
