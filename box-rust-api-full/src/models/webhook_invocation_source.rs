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
pub struct WebhookInvocationSource {
    /// The unique identifier that represent a folder.  The ID for any folder can be determined by visiting a folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folders/123` the `folder_id` is `123`.
    #[serde(rename = "id")]
    pub id: String,
    /// The HTTP `etag` of this folder. This can be used within some API endpoints in the `If-Match` and `If-None-Match` headers to only perform changes on the folder if (no) changes have happened.
    #[serde(rename = "etag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub etag: Option<Option<String>>,
    /// `folder`
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// A numeric identifier that represents the most recent user event that has been applied to this item.  This can be used in combination with the `GET /events`-endpoint to filter out user events that would have occurred before this identifier was read.  An example would be where a Box Drive-like application would fetch an item via the API, and then listen to incoming user events for changes to the item. The application would ignore any user events where the `sequence_id` in the event is smaller than or equal to the `sequence_id` in the originally fetched resource.
    #[serde(rename = "sequence_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sequence_id: Option<Option<String>>,
    /// The name of the folder.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The SHA1 hash of the file. This can be used to compare the contents of a file on Box with a local file.
    #[serde(rename = "sha1", skip_serializing_if = "Option::is_none")]
    pub sha1: Option<String>,
    #[serde(rename = "file_version", skip_serializing_if = "Option::is_none")]
    pub file_version: Option<Box<crate::models::FileMiniAllOfFileVersion>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<String>>,
    /// The folder size in bytes.  Be careful parsing this integer as its value can get very large.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "path_collection", skip_serializing_if = "Option::is_none")]
    pub path_collection: Option<Box<crate::models::FolderAllOfPathCollection>>,
    /// The date and time when the folder was created. This value may be `null` for some folders such as the root folder or the trash folder.
    #[serde(rename = "created_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Option<String>>,
    /// The date and time when the folder was last updated. This value may be `null` for some folders such as the root folder or the trash folder.
    #[serde(rename = "modified_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Option<String>>,
    /// The time at which this folder was put in the trash.
    #[serde(rename = "trashed_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub trashed_at: Option<Option<String>>,
    /// The time at which this folder is expected to be purged from the trash.
    #[serde(rename = "purged_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub purged_at: Option<Option<String>>,
    /// The date and time at which this folder was originally created.
    #[serde(rename = "content_created_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub content_created_at: Option<Option<String>>,
    /// The date and time at which this folder was last updated.
    #[serde(rename = "content_modified_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub content_modified_at: Option<Option<String>>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::FolderAllOfCreatedBy>>,
    #[serde(rename = "modified_by", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<Box<crate::models::FolderAllOfModifiedBy>>,
    #[serde(rename = "owned_by", skip_serializing_if = "Option::is_none")]
    pub owned_by: Option<Box<crate::models::FolderAllOfOwnedBy>>,
    #[serde(rename = "shared_link", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub shared_link: Option<Option<Box<crate::models::FolderAllOfSharedLink>>>,
    #[serde(rename = "parent", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent: Option<Option<Box<crate::models::FolderAllOfParent>>>,
    /// Defines if this item has been deleted or not.  * `active` when the item has is not in the trash * `trashed` when the item has been moved to the trash but not deleted * `deleted` when the item has been permanently deleted.
    #[serde(rename = "item_status", skip_serializing_if = "Option::is_none")]
    pub item_status: Option<ItemStatus>,
    #[serde(rename = "folder_upload_email", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub folder_upload_email: Option<Option<Box<crate::models::FolderAllOfFolderUploadEmail>>>,
    #[serde(rename = "item_collection", skip_serializing_if = "Option::is_none")]
    pub item_collection: Option<Box<crate::models::FolderAllOfItemCollection>>,
}

impl WebhookInvocationSource {
    pub fn new(id: String, r#type: RHashType) -> WebhookInvocationSource {
        WebhookInvocationSource {
            id,
            etag: None,
            r#type,
            sequence_id: None,
            name: None,
            sha1: None,
            file_version: None,
            description: None,
            size: None,
            path_collection: None,
            created_at: None,
            modified_at: None,
            trashed_at: None,
            purged_at: None,
            content_created_at: None,
            content_modified_at: None,
            created_by: None,
            modified_by: None,
            owned_by: None,
            shared_link: None,
            parent: None,
            item_status: None,
            folder_upload_email: None,
            item_collection: None,
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
/// Defines if this item has been deleted or not.  * `active` when the item has is not in the trash * `trashed` when the item has been moved to the trash but not deleted * `deleted` when the item has been permanently deleted.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ItemStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "trashed")]
    Trashed,
    #[serde(rename = "deleted")]
    Deleted,
}

impl Default for ItemStatus {
    fn default() -> ItemStatus {
        Self::Active
    }
}

