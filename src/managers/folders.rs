
use reqwest::{Body, multipart};
use serde::{Deserialize, Serialize};
use tokio_util::codec::{BytesCodec, FramedRead};
use tokio::fs::File;

use crate::box_api_client::BoxApiClient;
use crate::models::{FolderAllOfItemCollection, FolderFull};

pub async fn items(api: &BoxApiClient, folder_id: &String) -> FolderAllOfItemCollection {
    let response_string = api.get(format!("folders/{}/items", folder_id).as_str()).await.unwrap();
    serde_json::from_str(&response_string).unwrap()
}

pub async fn create(api: &BoxApiClient, request: &CreateFolderRequest) -> FolderFull {
    let response = api.post("folders", &request).await;
    serde_json::from_str(&response).unwrap()
}

pub async fn delete(api: &BoxApiClient, folder_id: &String, recursive: bool) -> () {
    let mut url = format!("folders/{}", folder_id);
    if recursive {
        url = format!("folders/{}?recursive=true", folder_id);
    }

    api.delete(url.as_str()).await
}

pub async fn get(api: &BoxApiClient, folder_id: &String) -> Option<FolderFull> {
    let response = api.get(format!("folders/{}", folder_id).as_str()).await;
    match response {
        None => None,
        Some(folder_string) => serde_json::from_str(&folder_string).unwrap()
    }
}

pub async fn upload_file(api: &BoxApiClient, file: File, attributes: &FileUploadAttributes) -> Option<String> {
    // // read file body stream
    let stream = FramedRead::new(file, BytesCodec::new());
    let file_body = Body::wrap_stream(stream);

    // // make form part of file
    let some_file = multipart::Part::stream(file_body)
        .file_name("image.jpeg")
        .mime_str("application/octet-stream").ok()?;

    // create the multipart form
    let form = multipart::Form::new()
        .text("attributes", serde_json::to_string(attributes).unwrap())
        .part("file", some_file);

    // send request
    api.multipart(form).await
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
struct Parent {
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFolderRequest {
    parent: Parent,
    name: String,
}

impl CreateFolderRequest {
    pub fn new(name: String, parent_folder_id: String) -> Self {
        Self {
            name,
            parent: Parent {
                id: parent_folder_id
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileUploadAttributes {
    parent: Parent,
    name: String,
}

impl FileUploadAttributes {
    pub fn new(name: String, parent_folder_id: &String) -> Self {
        Self {
            name,
            parent: Parent {
                id: parent_folder_id.clone()
            },
        }
    }
}