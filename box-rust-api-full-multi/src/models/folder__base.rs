/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// FolderBase : The bare basic representation of a folder, the minimal amount of fields returned when using the `fields` query parameter.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FolderBase {
    /// The unique identifier that represent a folder.  The ID for any folder can be determined by visiting a folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folders/123` the `folder_id` is `123`.
    #[serde(rename = "id")]
    pub id: String,
    /// The HTTP `etag` of this folder. This can be used within some API endpoints in the `If-Match` and `If-None-Match` headers to only perform changes on the folder if (no) changes have happened.
    #[serde(rename = "etag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub etag: Option<Option<String>>,
    /// `folder`
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl FolderBase {
    /// The bare basic representation of a folder, the minimal amount of fields returned when using the `fields` query parameter.
    pub fn new(id: String, r#type: RHashType) -> FolderBase {
        FolderBase {
            id,
            etag: None,
            r#type,
        }
    }
}

/// `folder`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "folder")]
    Folder,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Folder
    }
}

