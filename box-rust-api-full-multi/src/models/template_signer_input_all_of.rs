/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TemplateSignerInputAllOf {
    /// Type of input
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    /// Content type of input
    #[serde(rename = "content_type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<ContentType>,
    /// Whether or not the input is required.
    #[serde(rename = "is_required", skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    /// Index of page that the input is on.
    #[serde(rename = "page_index", skip_serializing_if = "Option::is_none")]
    pub page_index: Option<i32>,
    /// Document identifier.
    #[serde(rename = "document_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub document_id: Option<Option<String>>,
    /// When the input is of the type `dropdown` this values will be filled with all the dropdown options.
    #[serde(rename = "dropdown_choices", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub dropdown_choices: Option<Option<Vec<String>>>,
    /// When the input is of type `radio` they can be grouped to gather with this identifier.
    #[serde(rename = "group_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Option<String>>,
    #[serde(rename = "coordinates", skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<Box<crate::models::TemplateSignerInputAllOfCoordinates>>,
    #[serde(rename = "dimensions", skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Box<crate::models::TemplateSignerInputAllOfDimensions>>,
}

impl TemplateSignerInputAllOf {
    pub fn new() -> TemplateSignerInputAllOf {
        TemplateSignerInputAllOf {
            r#type: None,
            content_type: None,
            is_required: None,
            page_index: None,
            document_id: None,
            dropdown_choices: None,
            group_id: None,
            coordinates: None,
            dimensions: None,
        }
    }
}

/// Type of input
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "signature")]
    Signature,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "checkbox")]
    Checkbox,
    #[serde(rename = "radio")]
    Radio,
    #[serde(rename = "dropdown")]
    Dropdown,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Signature
    }
}
/// Content type of input
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContentType {
    #[serde(rename = "signature")]
    Signature,
    #[serde(rename = "initial")]
    Initial,
    #[serde(rename = "stamp")]
    Stamp,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "checkbox")]
    Checkbox,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "full_name")]
    FullName,
    #[serde(rename = "first_name")]
    FirstName,
    #[serde(rename = "last_name")]
    LastName,
    #[serde(rename = "company")]
    Company,
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "attachment")]
    Attachment,
    #[serde(rename = "radio")]
    Radio,
    #[serde(rename = "dropdown")]
    Dropdown,
}

impl Default for ContentType {
    fn default() -> ContentType {
        Self::Signature
    }
}

