/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// RepresentationsEntriesInner : A file representation



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RepresentationsEntriesInner {
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<Box<crate::models::RepresentationsEntriesInnerContent>>,
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<Box<crate::models::RepresentationsEntriesInnerInfo>>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Box<crate::models::RepresentationsEntriesInnerProperties>>,
    /// Indicates the file type of the returned representation.
    #[serde(rename = "representation", skip_serializing_if = "Option::is_none")]
    pub representation: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::RepresentationsEntriesInnerStatus>>,
}

impl RepresentationsEntriesInner {
    /// A file representation
    pub fn new() -> RepresentationsEntriesInner {
        RepresentationsEntriesInner {
            content: None,
            info: None,
            properties: None,
            representation: None,
            status: None,
        }
    }
}


