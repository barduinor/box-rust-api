/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// LegalHoldPolicyAssignment : Legal Hold Assignments are used to assign Legal Hold Policies to Users, Folders, Files, or File Versions.  Creating a Legal Hold Assignment puts a hold on the File-Versions that belong to the Assignment's 'apply-to' entity.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LegalHoldPolicyAssignment {
    /// The unique identifier for this legal hold assignment
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// `legal_hold_policy_assignment`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
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

impl LegalHoldPolicyAssignment {
    /// Legal Hold Assignments are used to assign Legal Hold Policies to Users, Folders, Files, or File Versions.  Creating a Legal Hold Assignment puts a hold on the File-Versions that belong to the Assignment's 'apply-to' entity.
    pub fn new() -> LegalHoldPolicyAssignment {
        LegalHoldPolicyAssignment {
            id: None,
            r#type: None,
            legal_hold_policy: None,
            assigned_to: None,
            assigned_by: None,
            assigned_at: None,
            deleted_at: None,
        }
    }
}

/// `legal_hold_policy_assignment`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "legal_hold_policy_assignment")]
    LegalHoldPolicyAssignment,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::LegalHoldPolicyAssignment
    }
}

