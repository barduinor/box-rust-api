/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// PostMetadataTemplatesSchemaClassificationsRequestFieldsInner : The `enum` field which holds all the valid classification values.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostMetadataTemplatesSchemaClassificationsRequestFieldsInner {
    /// `enum`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    /// `Box__Security__Classification__Key`
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<Key>,
    /// `Classification`
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<DisplayName>,
    /// `false`
    #[serde(rename = "hidden", skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// The actual list of classifications that are present on this template.
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<crate::models::PostMetadataTemplatesSchemaClassificationsRequestFieldsInnerOptionsInner>>,
}

impl PostMetadataTemplatesSchemaClassificationsRequestFieldsInner {
    /// The `enum` field which holds all the valid classification values.
    pub fn new() -> PostMetadataTemplatesSchemaClassificationsRequestFieldsInner {
        PostMetadataTemplatesSchemaClassificationsRequestFieldsInner {
            r#type: None,
            key: None,
            display_name: None,
            hidden: None,
            options: None,
        }
    }
}

/// `enum`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "enum")]
    Enum,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Enum
    }
}
/// `Box__Security__Classification__Key`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Key {
    #[serde(rename = "Box__Security__Classification__Key")]
    BoxSecurityClassificationKey,
}

impl Default for Key {
    fn default() -> Key {
        Self::BoxSecurityClassificationKey
    }
}
/// `Classification`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DisplayName {
    #[serde(rename = "Classification")]
    Classification,
}

impl Default for DisplayName {
    fn default() -> DisplayName {
        Self::Classification
    }
}

