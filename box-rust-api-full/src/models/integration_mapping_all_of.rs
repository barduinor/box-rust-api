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
pub struct IntegrationMappingAllOf {
    /// Mapping type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    #[serde(rename = "partner_item", skip_serializing_if = "Option::is_none")]
    pub partner_item: Option<Box<crate::models::IntegrationMappingAllOfPartnerItem>>,
    #[serde(rename = "box_item", skip_serializing_if = "Option::is_none")]
    pub box_item: Option<Box<crate::models::IntegrationMappingAllOfBoxItem>>,
    /// Identifies whether the mapping has been manually set (as opposed to being automatically created)
    #[serde(rename = "is_manually_created", skip_serializing_if = "Option::is_none")]
    pub is_manually_created: Option<bool>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::models::IntegrationMappingAllOfOptions>>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::IntegrationMappingAllOfCreatedBy>>,
    #[serde(rename = "modified_by", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<Box<crate::models::IntegrationMappingAllOfModifiedBy>>,
    /// When the integration mapping object was created
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the integration mapping object was last modified
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
}

impl IntegrationMappingAllOf {
    pub fn new() -> IntegrationMappingAllOf {
        IntegrationMappingAllOf {
            r#type: None,
            partner_item: None,
            box_item: None,
            is_manually_created: None,
            options: None,
            created_by: None,
            modified_by: None,
            created_at: None,
            modified_at: None,
        }
    }
}

/// Mapping type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "integration_mapping")]
    IntegrationMapping,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::IntegrationMapping
    }
}

