/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// Metadata : An instance of a metadata template, which has been applied to a file or folder.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Metadata {
    /// The identifier of the item that this metadata instance has been attached to. This combines the `type` and the `id` of the parent in the form `{type}_{id}`.
    #[serde(rename = "$parent", skip_serializing_if = "Option::is_none")]
    pub dollar_parent: Option<String>,
    /// The name of the template
    #[serde(rename = "$template", skip_serializing_if = "Option::is_none")]
    pub dollar_template: Option<String>,
    /// An ID for the scope in which this template has been applied. This will be `enterprise_{enterprise_id}` for templates defined for use in this enterprise, and `global` for general templates that are available to all enterprises using Box.
    #[serde(rename = "$scope", skip_serializing_if = "Option::is_none")]
    pub dollar_scope: Option<String>,
    /// The version of the metadata instance. This version starts at 0 and increases every time a user-defined property is modified.
    #[serde(rename = "$version", skip_serializing_if = "Option::is_none")]
    pub dollar_version: Option<i32>,
}

impl Metadata {
    /// An instance of a metadata template, which has been applied to a file or folder.
    pub fn new() -> Metadata {
        Metadata {
            dollar_parent: None,
            dollar_template: None,
            dollar_scope: None,
            dollar_version: None,
        }
    }
}


