/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// PostStoragePolicyAssignmentsRequestStoragePolicy : The storage policy to assign to the user or enterprise



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostStoragePolicyAssignmentsRequestStoragePolicy {
    /// The type to assign.
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// The ID of the storage policy to assign.
    #[serde(rename = "id")]
    pub id: String,
}

impl PostStoragePolicyAssignmentsRequestStoragePolicy {
    /// The storage policy to assign to the user or enterprise
    pub fn new(r#type: RHashType, id: String) -> PostStoragePolicyAssignmentsRequestStoragePolicy {
        PostStoragePolicyAssignmentsRequestStoragePolicy {
            r#type,
            id,
        }
    }
}

/// The type to assign.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "storage_policy")]
    StoragePolicy,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::StoragePolicy
    }
}

