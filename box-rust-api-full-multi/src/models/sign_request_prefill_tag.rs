/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// SignRequestPrefillTag : Prefill tags are used to prefill placeholders with signer input data. Only one value field can be included.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SignRequestPrefillTag {
    /// This references the ID of a specific tag contained in a file of the sign request.
    #[serde(rename = "document_tag_id", skip_serializing_if = "Option::is_none")]
    pub document_tag_id: Option<String>,
    /// Text prefill value
    #[serde(rename = "text_value", skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    /// Checkbox prefill value
    #[serde(rename = "checkbox_value", skip_serializing_if = "Option::is_none")]
    pub checkbox_value: Option<bool>,
    /// Date prefill value
    #[serde(rename = "date_value", skip_serializing_if = "Option::is_none")]
    pub date_value: Option<String>,
}

impl SignRequestPrefillTag {
    /// Prefill tags are used to prefill placeholders with signer input data. Only one value field can be included.
    pub fn new() -> SignRequestPrefillTag {
        SignRequestPrefillTag {
            document_tag_id: None,
            text_value: None,
            checkbox_value: None,
            date_value: None,
        }
    }
}

