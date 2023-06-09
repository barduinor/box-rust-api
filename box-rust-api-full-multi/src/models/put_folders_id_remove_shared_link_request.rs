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
pub struct PutFoldersIdRemoveSharedLinkRequest {
    /// By setting this value to `null`, the shared link is removed from the folder.
    #[serde(rename = "shared_link", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub shared_link: Option<Option<serde_json::Value>>,
}

impl PutFoldersIdRemoveSharedLinkRequest {
    pub fn new() -> PutFoldersIdRemoveSharedLinkRequest {
        PutFoldersIdRemoveSharedLinkRequest {
            shared_link: None,
        }
    }
}


