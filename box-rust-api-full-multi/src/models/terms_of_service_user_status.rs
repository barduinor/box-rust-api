/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// TermsOfServiceUserStatus : The association between a Terms of Service and a user



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TermsOfServiceUserStatus {
    /// The unique identifier for this terms of service user status
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// `terms_of_service_user_status`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    #[serde(rename = "tos", skip_serializing_if = "Option::is_none")]
    pub tos: Option<Box<crate::models::TermsOfServiceUserStatusTos>>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::TermsOfServiceUserStatusUser>>,
    /// If the user has accepted the terms of services
    #[serde(rename = "is_accepted", skip_serializing_if = "Option::is_none")]
    pub is_accepted: Option<bool>,
    /// When the legal item was created
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the legal item was modified.
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
}

impl TermsOfServiceUserStatus {
    /// The association between a Terms of Service and a user
    pub fn new() -> TermsOfServiceUserStatus {
        TermsOfServiceUserStatus {
            id: None,
            r#type: None,
            tos: None,
            user: None,
            is_accepted: None,
            created_at: None,
            modified_at: None,
        }
    }
}

/// `terms_of_service_user_status`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "terms_of_service_user_status")]
    TermsOfServiceUserStatus,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::TermsOfServiceUserStatus
    }
}

