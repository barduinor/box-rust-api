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
pub struct CollaborationAcceptanceRequirementsStatusStrongPasswordRequirement {
    /// Whether or not the enterprise that owns the content requires a strong password to collaborate on the content.
    #[serde(rename = "enterprise_has_strong_password_required_for_external_users", skip_serializing_if = "Option::is_none")]
    pub enterprise_has_strong_password_required_for_external_users: Option<bool>,
    /// Whether or not the user has a strong password set for their account. The field is `null` when a strong password is not required.
    #[serde(rename = "user_has_strong_password", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user_has_strong_password: Option<Option<bool>>,
}

impl CollaborationAcceptanceRequirementsStatusStrongPasswordRequirement {
    pub fn new() -> CollaborationAcceptanceRequirementsStatusStrongPasswordRequirement {
        CollaborationAcceptanceRequirementsStatusStrongPasswordRequirement {
            enterprise_has_strong_password_required_for_external_users: None,
            user_has_strong_password: None,
        }
    }
}


