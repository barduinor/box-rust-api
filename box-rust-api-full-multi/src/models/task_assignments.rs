/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// TaskAssignments : A list of task assignments



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TaskAssignments {
    /// The total number of items in this collection.
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[serde(rename = "entries", skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<crate::models::TaskAssignment>>,
}

impl TaskAssignments {
    /// A list of task assignments
    pub fn new() -> TaskAssignments {
        TaskAssignments {
            total_count: None,
            entries: None,
        }
    }
}


