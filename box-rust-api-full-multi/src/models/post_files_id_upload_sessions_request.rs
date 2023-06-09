/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostFilesIdUploadSessionsRequest {
    /// The total number of bytes of the file to be uploaded
    #[serde(rename = "file_size")]
    pub file_size: i64,
    /// The optional new name of new file
    #[serde(rename = "file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

impl PostFilesIdUploadSessionsRequest {
    pub fn new(file_size: i64) -> PostFilesIdUploadSessionsRequest {
        PostFilesIdUploadSessionsRequest {
            file_size,
            file_name: None,
        }
    }
}


