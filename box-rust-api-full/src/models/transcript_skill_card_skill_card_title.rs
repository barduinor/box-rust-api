/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// TranscriptSkillCardSkillCardTitle : The title of the card.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TranscriptSkillCardSkillCardTitle {
    /// An optional identifier for the title.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The actual title to show in the UI.
    #[serde(rename = "message")]
    pub message: String,
}

impl TranscriptSkillCardSkillCardTitle {
    /// The title of the card.
    pub fn new(message: String) -> TranscriptSkillCardSkillCardTitle {
        TranscriptSkillCardSkillCardTitle {
            code: None,
            message,
        }
    }
}


