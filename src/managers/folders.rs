use crate::box_api_client::BoxApiClient;
use crate::models::{FolderAllOfItemCollection, FolderFull};

use serde::{Deserialize, Serialize};

pub async fn items(api: &BoxApiClient, folder_id: &String) -> FolderAllOfItemCollection {
    let response_string = api.get(format!("folders/{}/items", folder_id).as_str()).await.unwrap();
    serde_json::from_str(&response_string).unwrap()
}

pub async fn create(api: &BoxApiClient, request: &CreateFolderRequest) -> FolderFull {
    let response = api.post("folders", &request).await;
    serde_json::from_str(&response).unwrap()
}

pub async fn delete(api: &BoxApiClient, folder_id: &String) -> () {
    api.delete(format!("folders/{}", folder_id).as_str()).await
}

pub async fn get(api: &BoxApiClient, folder_id: &String) -> Option<FolderFull> {
    let response = api.get(format!("folders/{}", folder_id).as_str()).await;
    match response {
        None => None,
        Some(folder_string) => serde_json::from_str(&folder_string).unwrap()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ItemType {
    #[serde(rename = "web_link")]
    WebLink,
    #[serde(rename = "folder")]
    Folder,
    #[serde(rename = "file")]
    File,
}

impl Default for ItemType {
    fn default() -> ItemType {
        Self::File
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFolderRequest {
    parent: Parent,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Parent {
    id: String
}

impl CreateFolderRequest {
    pub fn new(name: String, parent_folder_id: String) -> Self {
        Self {
            name,
            parent: Parent{
                id: parent_folder_id
            }
        }
    }

}
