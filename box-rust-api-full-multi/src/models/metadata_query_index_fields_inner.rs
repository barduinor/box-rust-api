/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// MetadataQueryIndexFieldsInner : The field which makes up the index.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetadataQueryIndexFieldsInner {
    /// The metadata template field key.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The sort direction of the field.
    #[serde(rename = "sort_direction", skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,
}

impl MetadataQueryIndexFieldsInner {
    /// The field which makes up the index.
    pub fn new() -> MetadataQueryIndexFieldsInner {
        MetadataQueryIndexFieldsInner {
            key: None,
            sort_direction: None,
        }
    }
}

/// The sort direction of the field.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SortDirection {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl Default for SortDirection {
    fn default() -> SortDirection {
        Self::Asc
    }
}

