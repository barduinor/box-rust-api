use reqwest::{multipart, Body};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Error;
use tokio::fs::File as TokioFile;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::box_api_client::BoxApiClient;
use crate::models::{Files, FolderAllOfItemCollection, FolderFull};

pub async fn items(api: &BoxApiClient, folder_id: &String) -> FolderAllOfItemCollection {
    let response_string = api
        .get(format!("folders/{}/items", folder_id).as_str())
        .await
        .unwrap();
    serde_json::from_str(&response_string).unwrap()
}

pub async fn create(api: &BoxApiClient, request: &CreateFolderRequest) -> FolderFull {
    let response = api.post("folders", &request).await;
    serde_json::from_str(&response).unwrap()
}

pub async fn delete(api: &BoxApiClient, folder_id: &String, recursive: bool) {
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
        Some(folder_string) => serde_json::from_str(&folder_string).unwrap(),
    }
}

//TODO: This should return a file object
pub async fn upload_file(
    api: &BoxApiClient,
    file: File,
    attributes: &FileUploadAttributes,
    // ) -> Option<String> {
) -> Result<crate::models::File, Error> {
    // // read file body stream
    let stream = FramedRead::new(TokioFile::from(file), BytesCodec::new());
    let file_body = Body::wrap_stream(stream);

    // // make form part of file
    let some_file = multipart::Part::stream(file_body)
        .file_name("image.jpeg")
        .mime_str("application/octet-stream")
        .ok();

    // create the multipart form
    let form = multipart::Form::new()
        .text("attributes", serde_json::to_string(attributes).unwrap())
        .part("file", some_file.unwrap());

    // send request
    let response = api.multipart(form).await.unwrap();

    let files: Files = serde_json::from_str(&response).unwrap();
    files
        .entries
        .unwrap()
        .into_iter()
        .next()
        .ok_or(Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No file found",
        )))
}

pub async fn download_file(
    api: &BoxApiClient,
    file_id: &String,
    destination: &mut File,
) -> Result<(), Error> {
    if (api
        .get_binary(&format!("files/{}/content", file_id), destination)
        .await)
        .is_ok()
    {}

    Ok(())
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
                id: parent_folder_id,
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
    pub fn new(name: String, parent_folder_id: &str) -> Self {
        Self {
            name,
            parent: Parent {
                id: parent_folder_id.to_owned(),
            },
        }
    }
}
