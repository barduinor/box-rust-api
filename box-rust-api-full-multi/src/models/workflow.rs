/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// Workflow : Box Relay Workflows are objects that represent a named collection of flows.  Your application must be authorized to use the `Manage Box Relay` application scope within the developer console in order to use this resource.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Workflow {
    /// The unique identifier for the workflow
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// `workflow`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    /// The name of the workflow
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description for a workflow.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Specifies if this workflow is enabled
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    /// A list of flows assigned to a workflow.
    #[serde(rename = "flows", skip_serializing_if = "Option::is_none")]
    pub flows: Option<Vec<crate::models::WorkflowAllOfFlows>>,
}

impl Workflow {
    /// Box Relay Workflows are objects that represent a named collection of flows.  Your application must be authorized to use the `Manage Box Relay` application scope within the developer console in order to use this resource.
    pub fn new() -> Workflow {
        Workflow {
            id: None,
            r#type: None,
            name: None,
            description: None,
            is_enabled: None,
            flows: None,
        }
    }
}

/// `workflow`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "workflow")]
    Workflow,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Workflow
    }
}

