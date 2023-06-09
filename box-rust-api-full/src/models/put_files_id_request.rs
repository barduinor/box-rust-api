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
pub struct PutFilesIdRequest {
    /// An optional different name for the file. This can be used to rename the file.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description for a file. This can be seen in the right-hand sidebar panel when viewing a file in the Box web app. Additionally, this index is used in the search index of the file, allowing users to find the file by the content in the description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<crate::models::PutFilesIdRequestParent>>,
    #[serde(rename = "shared_link", skip_serializing_if = "Option::is_none")]
    pub shared_link: Option<Box<crate::models::PutFilesIdRequestSharedLink>>,
    #[serde(rename = "lock", skip_serializing_if = "Option::is_none")]
    pub lock: Option<Box<crate::models::PutFilesIdRequestLock>>,
    /// The retention expiration timestamp for the given file. This date cannot be shortened once set on a file.
    #[serde(rename = "disposition_at", skip_serializing_if = "Option::is_none")]
    pub disposition_at: Option<String>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Box<crate::models::PutFilesIdRequestPermissions>>,
    /// The tags for this item. These tags are shown in the Box web app and mobile apps next to an item.  To add or remove a tag, retrieve the item's current tags, modify them, and then update this field.  There is a limit of 100 tags per item, and 10,000 unique tags per enterprise.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl PutFilesIdRequest {
    pub fn new() -> PutFilesIdRequest {
        PutFilesIdRequest {
            name: None,
            description: None,
            parent: None,
            shared_link: None,
            lock: None,
            disposition_at: None,
            permissions: None,
            tags: None,
        }
    }
}


