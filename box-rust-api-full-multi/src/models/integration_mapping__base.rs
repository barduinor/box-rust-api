/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// IntegrationMappingBase : A base representation of an integration mapping object.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IntegrationMappingBase {
    /// A unique identifier of a folder mapping (part of a composite key together with `integration_type`)
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Identifies the Box partner app, with which the mapping is associated. Currently only supports Slack. (part of the composite key together with `id`)
    #[serde(rename = "integration_type", skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<IntegrationType>,
}

impl IntegrationMappingBase {
    /// A base representation of an integration mapping object.
    pub fn new() -> IntegrationMappingBase {
        IntegrationMappingBase {
            id: None,
            integration_type: None,
        }
    }
}

/// Identifies the Box partner app, with which the mapping is associated. Currently only supports Slack. (part of the composite key together with `id`)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IntegrationType {
    #[serde(rename = "slack")]
    Slack,
}

impl Default for IntegrationType {
    fn default() -> IntegrationType {
        Self::Slack
    }
}

