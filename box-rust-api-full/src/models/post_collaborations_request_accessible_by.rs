/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// PostCollaborationsRequestAccessibleBy : The user or group to give access to the item.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostCollaborationsRequestAccessibleBy {
    /// The type of collaborator to invite.
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// The ID of the user or group.  Alternatively, use `login` to specify a user by email address.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The email address of the user to grant access to the item.  Alternatively, use `id` to specify a user by user ID.
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
}

impl PostCollaborationsRequestAccessibleBy {
    /// The user or group to give access to the item.
    pub fn new(r#type: RHashType) -> PostCollaborationsRequestAccessibleBy {
        PostCollaborationsRequestAccessibleBy {
            r#type,
            id: None,
            login: None,
        }
    }
}

/// The type of collaborator to invite.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "group")]
    Group,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::User
    }
}

