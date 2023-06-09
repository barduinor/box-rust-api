/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// PostStoragePolicyAssignmentsRequestAssignedTo : The user or enterprise to assign the storage policy to.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostStoragePolicyAssignmentsRequestAssignedTo {
    /// The type to assign the policy to.
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// The ID of the user or enterprise
    #[serde(rename = "id")]
    pub id: String,
}

impl PostStoragePolicyAssignmentsRequestAssignedTo {
    /// The user or enterprise to assign the storage policy to.
    pub fn new(r#type: RHashType, id: String) -> PostStoragePolicyAssignmentsRequestAssignedTo {
        PostStoragePolicyAssignmentsRequestAssignedTo {
            r#type,
            id,
        }
    }
}

/// The type to assign the policy to.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "enterprise")]
    Enterprise,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::User
    }
}

