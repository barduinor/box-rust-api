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
pub struct WorkflowFullAllOf {
    /// The date and time when the workflow was created on Box
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The date and time when the workflow was last updated on Box
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::WorkflowFullAllOfCreatedBy>>,
    #[serde(rename = "modified_by", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<Box<crate::models::WorkflowFullAllOfModifiedBy>>,
}

impl WorkflowFullAllOf {
    pub fn new() -> WorkflowFullAllOf {
        WorkflowFullAllOf {
            created_at: None,
            modified_at: None,
            created_by: None,
            modified_by: None,
        }
    }
}


