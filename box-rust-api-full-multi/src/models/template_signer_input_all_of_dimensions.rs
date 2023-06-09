/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// TemplateSignerInputAllOfDimensions : The size of the input.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TemplateSignerInputAllOfDimensions {
    /// Relative width to the page the input is on, ranging from 0 to 1.
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<f32>,
    /// Relative height to the page the input is on, ranging from 0 to 1.
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<f32>,
}

impl TemplateSignerInputAllOfDimensions {
    /// The size of the input.
    pub fn new() -> TemplateSignerInputAllOfDimensions {
        TemplateSignerInputAllOfDimensions {
            width: None,
            height: None,
        }
    }
}

