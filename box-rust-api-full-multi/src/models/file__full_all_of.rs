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
pub struct FileFullAllOf {
    /// The version number of this file
    #[serde(rename = "version_number", skip_serializing_if = "Option::is_none")]
    pub version_number: Option<String>,
    /// The number of comments on this file
    #[serde(rename = "comment_count", skip_serializing_if = "Option::is_none")]
    pub comment_count: Option<i32>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Box<crate::models::FileFullAllOfPermissions>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Box<crate::models::Array>>,
    #[serde(rename = "lock", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub lock: Option<Option<Box<crate::models::FileFullAllOfLock>>>,
    /// Indicates the (optional) file extension for this file. By default, this is set to an empty string.
    #[serde(rename = "extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    /// Indicates if the file is a package. Packages are commonly used by Mac Applications and can include iWork files.
    #[serde(rename = "is_package", skip_serializing_if = "Option::is_none")]
    pub is_package: Option<bool>,
    #[serde(rename = "expiring_embed_link", skip_serializing_if = "Option::is_none")]
    pub expiring_embed_link: Option<Box<crate::models::FileFullAllOfExpiringEmbedLink>>,
    #[serde(rename = "watermark_info", skip_serializing_if = "Option::is_none")]
    pub watermark_info: Option<Box<crate::models::FileFullAllOfWatermarkInfo>>,
    /// Specifies if the file can be accessed via the direct shared link or a shared link to a parent folder.
    #[serde(rename = "is_accessible_via_shared_link", skip_serializing_if = "Option::is_none")]
    pub is_accessible_via_shared_link: Option<IsAccessibleViaSharedLink>,
    /// A list of the types of roles that user can be invited at when sharing this file.
    #[serde(rename = "allowed_invitee_roles", skip_serializing_if = "Option::is_none")]
    pub allowed_invitee_roles: Option<Vec<AllowedInviteeRoles>>,
    /// Specifies if this file is owned by a user outside of the authenticated enterprise.
    #[serde(rename = "is_externally_owned", skip_serializing_if = "Option::is_none")]
    pub is_externally_owned: Option<bool>,
    /// Specifies if this file has any other collaborators.
    #[serde(rename = "has_collaborations", skip_serializing_if = "Option::is_none")]
    pub has_collaborations: Option<bool>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::Map>>,
    /// When the file will automatically be deleted
    #[serde(rename = "expires_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Option<String>>,
    #[serde(rename = "representations", skip_serializing_if = "Option::is_none")]
    pub representations: Option<Box<crate::models::FileFullAllOfRepresentations>>,
    #[serde(rename = "classification", skip_serializing_if = "Option::is_none")]
    pub classification: Option<Box<crate::models::FileFullAllOfClassification>>,
    /// The display name of the user that uploaded the file. In most cases this is the name of the user logged in at the time of the upload.  If the file was uploaded using a File Request form that requires the user to provide an email address, this field is populated with that email address. If an email address was not required in the File Request form, this field is set to return a value of `File Request`.  In all other anonymous cases where no email was provided this field will default to a value of `Someone`.
    #[serde(rename = "uploader_display_name", skip_serializing_if = "Option::is_none")]
    pub uploader_display_name: Option<String>,
    /// The retention expiration timestamp for the given file
    #[serde(rename = "disposition_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub disposition_at: Option<Option<String>>,
    /// A list of the types of roles that user can be invited at when sharing this file.
    #[serde(rename = "shared_link_permission_options", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub shared_link_permission_options: Option<Option<Vec<SharedLinkPermissionOptions>>>,
}

impl FileFullAllOf {
    pub fn new() -> FileFullAllOf {
        FileFullAllOf {
            version_number: None,
            comment_count: None,
            permissions: None,
            tags: None,
            lock: None,
            extension: None,
            is_package: None,
            expiring_embed_link: None,
            watermark_info: None,
            is_accessible_via_shared_link: None,
            allowed_invitee_roles: None,
            is_externally_owned: None,
            has_collaborations: None,
            metadata: None,
            expires_at: None,
            representations: None,
            classification: None,
            uploader_display_name: None,
            disposition_at: None,
            shared_link_permission_options: None,
        }
    }
}

/// Specifies if the file can be accessed via the direct shared link or a shared link to a parent folder.
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
/// A list of the types of roles that user can be invited at when sharing this file.
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
/// A list of the types of roles that user can be invited at when sharing this file.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SharedLinkPermissionOptions {
    #[serde(rename = "can_preview")]
    Preview,
    #[serde(rename = "can_download")]
    Download,
    #[serde(rename = "can_edit")]
    Edit,
}

impl Default for SharedLinkPermissionOptions {
    fn default() -> SharedLinkPermissionOptions {
        Self::Preview
    }
}

