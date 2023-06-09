/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// TermsOfServices : A list of terms of services



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TermsOfServices {
    /// The total number of objects.
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[serde(rename = "entries", skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<crate::models::TermsOfService>>,
}

impl TermsOfServices {
    /// A list of terms of services
    pub fn new() -> TermsOfServices {
        TermsOfServices {
            total_count: None,
            entries: None,
        }
    }
}


