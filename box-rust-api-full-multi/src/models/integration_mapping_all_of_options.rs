/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// IntegrationMappingAllOfOptions : Integration mapping options for Slack



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IntegrationMappingAllOfOptions {
    /// Indicates whether or not channel member access to the underlying box item should be automatically managed. Depending on type of channel, access is managed through creating collaborations or shared links.
    #[serde(rename = "is_access_management_disabled", skip_serializing_if = "Option::is_none")]
    pub is_access_management_disabled: Option<bool>,
}

impl IntegrationMappingAllOfOptions {
    /// Integration mapping options for Slack
    pub fn new() -> IntegrationMappingAllOfOptions {
        IntegrationMappingAllOfOptions {
            is_access_management_disabled: None,
        }
    }
}


