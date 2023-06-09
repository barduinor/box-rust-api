/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// CommentFull : Comments are messages created on files. Comments can be made independently or created as responses to other comments



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CommentFull {
    /// The unique identifier for this comment.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// `comment`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    /// Whether or not this comment is a reply to another comment
    #[serde(rename = "is_reply_comment", skip_serializing_if = "Option::is_none")]
    pub is_reply_comment: Option<bool>,
    /// The text of the comment, as provided by the user
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::CommentAllOfCreatedBy>>,
    /// The time this comment was created
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The time this comment was last modified
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(rename = "item", skip_serializing_if = "Option::is_none")]
    pub item: Option<Box<crate::models::CommentAllOfItem>>,
    /// The string representing the comment text with @mentions included. @mention format is @[id:username] where `id` is user's Box ID and `username` is their display name.
    #[serde(rename = "tagged_message", skip_serializing_if = "Option::is_none")]
    pub tagged_message: Option<String>,
}

impl CommentFull {
    /// Comments are messages created on files. Comments can be made independently or created as responses to other comments
    pub fn new() -> CommentFull {
        CommentFull {
            id: None,
            r#type: None,
            is_reply_comment: None,
            message: None,
            created_by: None,
            created_at: None,
            modified_at: None,
            item: None,
            tagged_message: None,
        }
    }
}

/// `comment`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "comment")]
    Comment,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Comment
    }
}

