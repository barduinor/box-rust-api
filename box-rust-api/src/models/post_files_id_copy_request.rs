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
pub struct PostFilesIdCopyRequest {
    /// An optional new name for the copied file.  There are some restrictions to the file name. Names containing non-printable ASCII characters, forward and backward slashes (`/`, `\\`), and protected names like `.` and `..` are automatically sanitized by removing the non-allowed characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// An optional ID of the specific file version to copy.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "parent")]
    pub parent: Box<crate::models::PostFilesIdCopyRequestParent>,
}

impl PostFilesIdCopyRequest {
    pub fn new(parent: crate::models::PostFilesIdCopyRequestParent) -> PostFilesIdCopyRequest {
        PostFilesIdCopyRequest {
            name: None,
            version: None,
            parent: Box::new(parent),
        }
    }
}

