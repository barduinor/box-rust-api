/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// FolderFull : A full representation of a folder, as can be returned from any folder API endpoints by default

// use serde_json::Value::Array;
// use serde_json::Map;
// use serde_with::Map;
// use core::iter::Map;
// use std::iter::Map;
// use crate::models::folder__full::Map;

use serde::{Deserialize, Serialize};
use crate::models;
use crate::models::{ItemType, ItemType::Folder, FolderAllOfPathCollection, FolderAllOfModifiedBy, FolderAllOfOwnedBy};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FolderFull {
    /// The unique identifier that represent a folder.  The ID for any folder can be determined by visiting a folder in the web application and copying the ID from the URL. For example, for the URL `https://_*.app.box.com/folders/123` the `folder_id` is `123`.
    #[serde(rename = "id")]
    pub id: String,
    /// The HTTP `etag` of this folder. This can be used within some API endpoints in the `If-Match` and `If-None-Match` headers to only perform changes on the folder if (no) changes have happened.
    #[serde(rename = "etag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub etag: Option<Option<String>>,
    /// `folder`
    #[serde(rename = "type")]
    pub item_type: ItemType,
    /// The name of the folder.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A numeric identifier that represents the most recent user event that has been applied to this item.  This can be used in combination with the `GET /events`-endpoint to filter out user events that would have occurred before this identifier was read.  An example would be where a Box Drive-like application would fetch an item via the API, and then listen to incoming user events for changes to the item. The application would ignore any user events where the `sequence_id` in the event is smaller than or equal to the `sequence_id` in the originally fetched resource.
    #[serde(rename = "sequence_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sequence_id: Option<Option<String>>,
    /// The date and time when the folder was created. This value may be `null` for some folders such as the root folder or the trash folder.
    #[serde(rename = "created_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Option<String>>,
    /// The date and time when the folder was last updated. This value may be `null` for some folders such as the root folder or the trash folder.
    #[serde(rename = "modified_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<Option<String>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<String>>,
    /// The folder size in bytes.  Be careful parsing this integer as its value can get very large.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "path_collection", skip_serializing_if = "Option::is_none")]
    pub path_collection: Option<Box<FolderAllOfPathCollection>>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<models::FolderAllOfCreatedBy>>,
    #[serde(rename = "modified_by", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<Box<FolderAllOfModifiedBy>>,
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
    #[serde(rename = "owned_by", skip_serializing_if = "Option::is_none")]
    pub owned_by: Option<Box<FolderAllOfOwnedBy>>,
    #[serde(rename = "shared_link", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub shared_link: Option<Option<Box<models::FolderAllOfSharedLink>>>,
    #[serde(rename = "folder_upload_email", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub folder_upload_email: Option<Option<Box<models::FolderAllOfFolderUploadEmail>>>,
    #[serde(rename = "parent", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent: Option<Option<Box<models::FolderAllOfParent>>>,
    /// Defines if this item has been deleted or not.  * `active` when the item has is not in the trash * `trashed` when the item has been moved to the trash but not deleted * `deleted` when the item has been permanently deleted.
    #[serde(rename = "item_status", skip_serializing_if = "Option::is_none")]
    pub item_status: Option<ItemStatus>,
    #[serde(rename = "item_collection", skip_serializing_if = "Option::is_none")]
    pub item_collection: Option<Box<models::FolderAllOfItemCollection>>,
    /// Specifies whether a folder should be synced to a user's device or not. This is used by Box Sync (discontinued) and is not used by Box Drive.
    #[serde(rename = "sync_state", skip_serializing_if = "Option::is_none")]
    pub sync_state: Option<SyncState>,
    /// Specifies if this folder has any other collaborators.
    #[serde(rename = "has_collaborations", skip_serializing_if = "Option::is_none")]
    pub has_collaborations: Option<bool>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Box<models::FolderFullAllOfPermissions>>,

    // TODO: open api generator miss match???
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    //pub tags: Option<Box<crate::models::Array>>,
    pub tags: Option<Box<serde_json::Value>>,
    // TODO: test tags above

    #[serde(rename = "can_non_owners_invite", skip_serializing_if = "Option::is_none")]
    pub can_non_owners_invite: Option<Box<bool>>,
    /// Specifies if this folder is owned by a user outside of the authenticated enterprise.
    #[serde(rename = "is_externally_owned", skip_serializing_if = "Option::is_none")]
    pub is_externally_owned: Option<bool>,

    // // TODO: open api generator miss match???
    // #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    // // pub metadata: Option<Box<crate::models::Map>>,
    // pub metadata: Option<Box<Map>>,
    // // TODO: test metadata above

    #[serde(rename = "is_collaboration_restricted_to_enterprise", skip_serializing_if = "Option::is_none")]
    pub is_collaboration_restricted_to_enterprise: Option<Box<bool>>,
    /// A list of access levels that are available for this folder.  For some folders, like the root folder, this will always be an empty list as sharing is not allowed at that level.
    #[serde(rename = "allowed_shared_link_access_levels", skip_serializing_if = "Option::is_none")]
    pub allowed_shared_link_access_levels: Option<Vec<AllowedSharedLinkAccessLevels>>,
    /// A list of the types of roles that user can be invited at when sharing this folder.
    #[serde(rename = "allowed_invitee_roles", skip_serializing_if = "Option::is_none")]
    pub allowed_invitee_roles: Option<Vec<AllowedInviteeRoles>>,
    #[serde(rename = "watermark_info", skip_serializing_if = "Option::is_none")]
    pub watermark_info: Option<Box<models::FolderFullAllOfWatermarkInfo>>,
    /// Specifies if the folder can be accessed with the direct shared link or a shared link to a parent folder.
    #[serde(rename = "is_accessible_via_shared_link", skip_serializing_if = "Option::is_none")]
    pub is_accessible_via_shared_link: Option<IsAccessibleViaSharedLink>,
    /// Specifies if collaborators who are not owners of this folder are restricted from viewing other collaborations on this folder.  It also restricts non-owners from inviting new collaborators.
    #[serde(rename = "can_non_owners_view_collaborators", skip_serializing_if = "Option::is_none")]
    pub can_non_owners_view_collaborators: Option<bool>,
    #[serde(rename = "classification", skip_serializing_if = "Option::is_none")]
    pub classification: Option<Box<models::FolderFullAllOfClassification>>,
}

impl FolderFull {
    /// A full representation of a folder, as can be returned from any folder API endpoints by default
    pub fn new(id: String) -> FolderFull {
        FolderFull {
            id,
            etag: None,
            item_type: Folder,
            name: None,
            sequence_id: None,
            created_at: None,
            modified_at: None,
            description: None,
            size: None,
            path_collection: None,
            created_by: None,
            modified_by: None,
            trashed_at: None,
            purged_at: None,
            content_created_at: None,
            content_modified_at: None,
            owned_by: None,
            shared_link: None,
            folder_upload_email: None,
            parent: None,
            item_status: None,
            item_collection: None,
            sync_state: None,
            has_collaborations: None,
            permissions: None,
            tags: None,
            can_non_owners_invite: None,
            is_externally_owned: None,

            // TODO: re instate metadata
            // metadata: None,

            is_collaboration_restricted_to_enterprise: None,
            allowed_shared_link_access_levels: None,
            allowed_invitee_roles: None,
            watermark_info: None,
            is_accessible_via_shared_link: None,
            can_non_owners_view_collaborators: None,
            classification: None,
        }
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
/// Specifies whether a folder should be synced to a user's device or not. This is used by Box Sync (discontinued) and is not used by Box Drive.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyncState {
    #[serde(rename = "synced")]
    Synced,
    #[serde(rename = "not_synced")]
    NotSynced,
    #[serde(rename = "partially_synced")]
    PartiallySynced,
}

impl Default for SyncState {
    fn default() -> SyncState {
        Self::Synced
    }
}
/// A list of access levels that are available for this folder.  For some folders, like the root folder, this will always be an empty list as sharing is not allowed at that level.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AllowedSharedLinkAccessLevels {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "company")]
    Company,
    #[serde(rename = "collaborators")]
    Collaborators,
}

impl Default for AllowedSharedLinkAccessLevels {
    fn default() -> AllowedSharedLinkAccessLevels {
        Self::Open
    }
}
/// A list of the types of roles that user can be invited at when sharing this folder.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AllowedInviteeRoles {
    #[serde(rename = "editor")]
    Editor,
    #[serde(rename = "viewer")]
    Viewer,
    #[serde(rename = "previewer")]
    Previewer,
    #[serde(rename = "uploader")]
    Uploader,
    #[serde(rename = "previewer uploader")]
    PreviewerUploader,
    #[serde(rename = "viewer uploader")]
    ViewerUploader,
    #[serde(rename = "co-owner")]
    CoOwner,
}

impl Default for AllowedInviteeRoles {
    fn default() -> AllowedInviteeRoles {
        Self::Editor
    }
}
/// Specifies if the folder can be accessed with the direct shared link or a shared link to a parent folder.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IsAccessibleViaSharedLink {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
}

impl Default for IsAccessibleViaSharedLink {
    fn default() -> IsAccessibleViaSharedLink {
        Self::True
    }
}

