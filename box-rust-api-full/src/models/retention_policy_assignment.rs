/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// RetentionPolicyAssignment : A retention assignment represents a rule specifying the files a retention policy retains. Assignments can retain files based on their folder or metadata, or hold all files in the enterprise.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RetentionPolicyAssignment {
    /// The unique identifier for a retention policy assignment.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// `retention_policy_assignment`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    #[serde(rename = "retention_policy", skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<Box<crate::models::RetentionPolicyAssignmentRetentionPolicy>>,
    #[serde(rename = "assigned_to", skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<Box<crate::models::RetentionPolicyAssignmentAssignedTo>>,
    /// An array of field objects. Values are only returned if the `assigned_to` type is `metadata_template`. Otherwise, the array is blank.
    #[serde(rename = "filter_fields", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub filter_fields: Option<Option<Vec<crate::models::RetentionPolicyAssignmentFilterFieldsInner>>>,
    #[serde(rename = "assigned_by", skip_serializing_if = "Option::is_none")]
    pub assigned_by: Option<Box<crate::models::RetentionPolicyAssignmentAssignedBy>>,
    /// When the retention policy assignment object was created.
    #[serde(rename = "assigned_at", skip_serializing_if = "Option::is_none")]
    pub assigned_at: Option<String>,
    /// The date the retention policy assignment begins. If the `assigned_to` type is `metadata_template`, this field can be a date field's metadata attribute key id.
    #[serde(rename = "start_date_field", skip_serializing_if = "Option::is_none")]
    pub start_date_field: Option<String>,
}

impl RetentionPolicyAssignment {
    /// A retention assignment represents a rule specifying the files a retention policy retains. Assignments can retain files based on their folder or metadata, or hold all files in the enterprise.
    pub fn new() -> RetentionPolicyAssignment {
        RetentionPolicyAssignment {
            id: None,
            r#type: None,
            retention_policy: None,
            assigned_to: None,
            filter_fields: None,
            assigned_by: None,
            assigned_at: None,
            start_date_field: None,
        }
    }
}

/// `retention_policy_assignment`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "retention_policy_assignment")]
    RetentionPolicyAssignment,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::RetentionPolicyAssignment
    }
}

