/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// GroupMembership : Membership is used to signify that a user is part of a group.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GroupMembership {
    /// The unique identifier for this group membership
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// `group_membership`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::GroupMembershipUser>>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<Box<crate::models::GroupMembershipGroup>>,
    /// The role of the user in the group.
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    /// The time this membership was created.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The time this membership was last modified.
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
}

impl GroupMembership {
    /// Membership is used to signify that a user is part of a group.
    pub fn new() -> GroupMembership {
        GroupMembership {
            id: None,
            r#type: None,
            user: None,
            group: None,
            role: None,
            created_at: None,
            modified_at: None,
        }
    }
}

/// `group_membership`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "group_membership")]
    GroupMembership,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::GroupMembership
    }
}
/// The role of the user in the group.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "admin")]
    Admin,
}

impl Default for Role {
    fn default() -> Role {
        Self::Member
    }
}

