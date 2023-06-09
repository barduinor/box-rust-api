/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// PathCollection : A list of parent folders for an item.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PathCollection {
    /// The number of folders in this list.
    #[serde(rename = "total_count")]
    pub total_count: i64,
    /// The parent folders for this item
    #[serde(rename = "entries")]
    pub entries: Vec<crate::models::FolderMini>,
}

impl PathCollection {
    /// A list of parent folders for an item.
    pub fn new(total_count: i64, entries: Vec<crate::models::FolderMini>) -> PathCollection {
        PathCollection {
            total_count,
            entries,
        }
    }
}


