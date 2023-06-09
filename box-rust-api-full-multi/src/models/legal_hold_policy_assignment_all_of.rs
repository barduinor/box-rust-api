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
pub struct LegalHoldPolicyAssignmentAllOf {
    #[serde(rename = "legal_hold_policy", skip_serializing_if = "Option::is_none")]
    pub legal_hold_policy: Option<Box<crate::models::LegalHoldPolicyAssignmentAllOfLegalHoldPolicy>>,
    #[serde(rename = "assigned_to", skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<Box<crate::models::LegalHoldPolicyAssignmentAllOfAssignedTo>>,
    #[serde(rename = "assigned_by", skip_serializing_if = "Option::is_none")]
    pub assigned_by: Option<Box<crate::models::LegalHoldPolicyAssignmentAllOfAssignedBy>>,
    /// When the legal hold policy assignment object was created
    #[serde(rename = "assigned_at", skip_serializing_if = "Option::is_none")]
    pub assigned_at: Option<String>,
    /// When the assignment release request was sent. (Because it can take time for an assignment to fully delete, this isn't quite the same time that the assignment is fully deleted). If null, Assignment was not deleted.
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
}

impl LegalHoldPolicyAssignmentAllOf {
    pub fn new() -> LegalHoldPolicyAssignmentAllOf {
        LegalHoldPolicyAssignmentAllOf {
            legal_hold_policy: None,
            assigned_to: None,
            assigned_by: None,
            assigned_at: None,
            deleted_at: None,
        }
    }
}


