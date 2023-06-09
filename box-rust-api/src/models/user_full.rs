/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// UserFull : A full representation of a user, as can be returned from any user API endpoint.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserFull {
    /// The unique identifier for this user
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// `user`
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// The display name of this user
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The primary email address of this user
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    /// When the user object was created
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the user object was last modified
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    /// The language of the user, formatted in modified version of the [ISO 639-1](/guides/api-calls/language-codes) format.
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// The user's timezone
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// The user’s total available space amount in bytes
    #[serde(rename = "space_amount", skip_serializing_if = "Option::is_none")]
    pub space_amount: Option<i64>,
    /// The amount of space in use by the user
    #[serde(rename = "space_used", skip_serializing_if = "Option::is_none")]
    pub space_used: Option<i64>,
    /// The maximum individual file size in bytes the user can have
    #[serde(rename = "max_upload_size", skip_serializing_if = "Option::is_none")]
    pub max_upload_size: Option<i64>,
    /// The user's account status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The user’s job title
    #[serde(rename = "job_title", skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    /// The user’s phone number
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The user’s address
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// URL of the user’s avatar image
    #[serde(rename = "avatar_url", skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    // TODO:Fix serialization of notification_email
    // #[serde(rename = "notification_email", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    // pub notification_email: Option<Option<Box<crate::models::UserAllOfNotificationEmail>>>,
    /// The user’s enterprise role
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    /// Tracking codes allow an admin to generate reports from the admin console and assign an attribute to a specific group of users. This setting must be enabled for an enterprise before it can be used.
    #[serde(rename = "tracking_codes", skip_serializing_if = "Option::is_none")]
    pub tracking_codes: Option<Vec<crate::models::TrackingCode>>,
    /// Whether the user can see other enterprise users in their contact list
    #[serde(rename = "can_see_managed_users", skip_serializing_if = "Option::is_none")]
    pub can_see_managed_users: Option<bool>,
    /// Whether the user can use Box Sync
    #[serde(rename = "is_sync_enabled", skip_serializing_if = "Option::is_none")]
    pub is_sync_enabled: Option<bool>,
    /// Whether the user is allowed to collaborate with users outside their enterprise
    #[serde(rename = "is_external_collab_restricted", skip_serializing_if = "Option::is_none")]
    pub is_external_collab_restricted: Option<bool>,
    /// Whether to exempt the user from Enterprise device limits
    #[serde(rename = "is_exempt_from_device_limits", skip_serializing_if = "Option::is_none")]
    pub is_exempt_from_device_limits: Option<bool>,
    /// Whether the user must use two-factor authentication
    #[serde(rename = "is_exempt_from_login_verification", skip_serializing_if = "Option::is_none")]
    pub is_exempt_from_login_verification: Option<bool>,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<crate::models::UserFullAllOfEnterprise>>,
    /// Tags for all files and folders owned by the user. Values returned will only contain tags that were set by the requester.
    #[serde(rename = "my_tags", skip_serializing_if = "Option::is_none")]
    pub my_tags: Option<Vec<String>>,
    /// The root (protocol, subdomain, domain) of any links that need to be generated for the user
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Whether the user is an App User
    #[serde(rename = "is_platform_access_only", skip_serializing_if = "Option::is_none")]
    pub is_platform_access_only: Option<bool>,
    /// An external identifier for an app user, which can be used to look up the user. This can be used to tie user IDs from external identity providers to Box users.
    #[serde(rename = "external_app_user_id", skip_serializing_if = "Option::is_none")]
    pub external_app_user_id: Option<String>,
}

impl UserFull {
    /// A full representation of a user, as can be returned from any user API endpoint.
    pub fn new(r#type: RHashType) -> UserFull {
        UserFull {
            id: None,
            r#type,
            name: None,
            login: None,
            created_at: None,
            modified_at: None,
            language: None,
            timezone: None,
            space_amount: None,
            space_used: None,
            max_upload_size: None,
            status: None,
            job_title: None,
            phone: None,
            address: None,
            avatar_url: None,
            // TODO:Fix serialization of notification_email
            // notification_email: None,
            role: None,
            tracking_codes: None,
            can_see_managed_users: None,
            is_sync_enabled: None,
            is_external_collab_restricted: None,
            is_exempt_from_device_limits: None,
            is_exempt_from_login_verification: None,
            enterprise: None,
            my_tags: None,
            hostname: None,
            is_platform_access_only: None,
            external_app_user_id: None,
        }
    }
}

/// `user`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "user")]
    User,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::User
    }
}
/// The user's account status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "cannot_delete_edit")]
    CannotDeleteEdit,
    #[serde(rename = "cannot_delete_edit_upload")]
    CannotDeleteEditUpload,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}
/// The user’s enterprise role
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "coadmin")]
    Coadmin,
    #[serde(rename = "user")]
    User,
}

impl Default for Role {
    fn default() -> Role {
        Self::Admin
    }
}

