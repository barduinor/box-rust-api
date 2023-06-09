/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// SearchResultsWithSharedLinks : A list of files, folders and web links that matched the search query, including the additional information about any shared links through which the item has been shared with the user.  This response format is only returned when the `include_recent_shared_links` query parameter has been set to `true`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SearchResultsWithSharedLinks {
    /// One greater than the offset of the last entry in the search results. The total number of entries in the collection may be less than `total_count`.
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// The limit that was used for this search. This will be the same as the `limit` query parameter unless that value exceeded the maximum value allowed.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The 0-based offset of the first entry in this set. This will be the same as the `offset` query parameter used.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// The search results for the query provided, including the additional information about any shared links through which the item has been shared with the user.
    #[serde(rename = "entries", skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<crate::models::SearchResultWithSharedLink>>,
}

impl SearchResultsWithSharedLinks {
    /// A list of files, folders and web links that matched the search query, including the additional information about any shared links through which the item has been shared with the user.  This response format is only returned when the `include_recent_shared_links` query parameter has been set to `true`.
    pub fn new() -> SearchResultsWithSharedLinks {
        SearchResultsWithSharedLinks {
            total_count: None,
            limit: None,
            offset: None,
            entries: None,
        }
    }
}


