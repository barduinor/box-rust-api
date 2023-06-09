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
pub struct FolderFullAllOf {
    /// Specifies whether a folder should be synced to a user's device or not. This is used by Box Sync (discontinued) and is not used by Box Drive.
    #[serde(rename = "sync_state", skip_serializing_if = "Option::is_none")]
    pub sync_state: Option<SyncState>,
    /// Specifies if this folder has any other collaborators.
    #[serde(rename = "has_collaborations", skip_serializing_if = "Option::is_none")]
    pub has_collaborations: Option<bool>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Box<crate::models::FolderFullAllOfPermissions>>,
    //
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    // pub tags: Option<Box<crate::models::Array>>,
    pub tags: Option<Box<serde_json::Value>>,
    //
    #[serde(
        rename = "can_non_owners_invite",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_non_owners_invite: Option<Box<bool>>,
    /// Specifies if this folder is owned by a user outside of the authenticated enterprise.
    #[serde(
        rename = "is_externally_owned",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_externally_owned: Option<bool>,
    //
    // TODO: Fix metadata serde
    // #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    // pub metadata: Option<Box<crate::models::Map>>,
    //
    #[serde(
        rename = "is_collaboration_restricted_to_enterprise",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_collaboration_restricted_to_enterprise: Option<Box<bool>>,
    /// A list of access levels that are available for this folder.  For some folders, like the root folder, this will always be an empty list as sharing is not allowed at that level.
    #[serde(
        rename = "allowed_shared_link_access_levels",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_shared_link_access_levels: Option<Vec<AllowedSharedLinkAccessLevels>>,
    /// A list of the types of roles that user can be invited at when sharing this folder.
    #[serde(
        rename = "allowed_invitee_roles",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_invitee_roles: Option<Vec<AllowedInviteeRoles>>,
    #[serde(rename = "watermark_info", skip_serializing_if = "Option::is_none")]
    pub watermark_info: Option<Box<crate::models::FolderFullAllOfWatermarkInfo>>,
    /// Specifies if the folder can be accessed with the direct shared link or a shared link to a parent folder.
    #[serde(
        rename = "is_accessible_via_shared_link",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_accessible_via_shared_link: Option<IsAccessibleViaSharedLink>,
    /// Specifies if collaborators who are not owners of this folder are restricted from viewing other collaborations on this folder.  It also restricts non-owners from inviting new collaborators.
    #[serde(
        rename = "can_non_owners_view_collaborators",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_non_owners_view_collaborators: Option<bool>,
    #[serde(rename = "classification", skip_serializing_if = "Option::is_none")]
    pub classification: Option<Box<crate::models::FolderFullAllOfClassification>>,
}

impl FolderFullAllOf {
    pub fn new() -> FolderFullAllOf {
        FolderFullAllOf {
            sync_state: None,
            has_collaborations: None,
            permissions: None,
            tags: None,
            can_non_owners_invite: None,
            is_externally_owned: None,
            //
            // TODO: Fix metadata serde
            // metadata: None,
            //
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
