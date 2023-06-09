/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// ClassificationTemplate : A metadata template that holds the security classifications defined by an enterprise.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ClassificationTemplate {
    /// The ID of the classification template.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// `metadata_template`
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// The scope of the classification template. This is in the format `enterprise_{id}` where the `id` is the enterprise ID.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// `securityClassification-6VMVochwUWo`
    #[serde(rename = "templateKey", skip_serializing_if = "Option::is_none")]
    pub template_key: Option<TemplateKey>,
    /// The name of this template as shown in web and mobile interfaces.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<DisplayName>,
    /// This template is always available in web and mobile interfaces.
    #[serde(rename = "hidden", skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// Classifications are always copied along when the file or folder is copied.
    #[serde(rename = "copyInstanceOnItemCopy", skip_serializing_if = "Option::is_none")]
    pub copy_instance_on_item_copy: Option<bool>,
    /// A list of fields for this classification template. This includes only one field, the `Box__Security__Classification__Key`, which defines the different classifications available in this enterprise.
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<crate::models::ClassificationTemplateFieldsInner>>,
}

impl ClassificationTemplate {
    /// A metadata template that holds the security classifications defined by an enterprise.
    pub fn new(r#type: RHashType) -> ClassificationTemplate {
        ClassificationTemplate {
            id: None,
            r#type,
            scope: None,
            template_key: None,
            display_name: None,
            hidden: None,
            copy_instance_on_item_copy: None,
            fields: None,
        }
    }
}

/// `metadata_template`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "metadata_template")]
    MetadataTemplate,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::MetadataTemplate
    }
}
/// `securityClassification-6VMVochwUWo`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TemplateKey {
    #[serde(rename = "securityClassification-6VMVochwUWo")]
    SecurityClassification6VmVochwUwo,
}

impl Default for TemplateKey {
    fn default() -> TemplateKey {
        Self::SecurityClassification6VmVochwUwo
    }
}
/// The name of this template as shown in web and mobile interfaces.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DisplayName {
    #[serde(rename = "Classification")]
    Classification,
}

impl Default for DisplayName {
    fn default() -> DisplayName {
        Self::Classification
    }
}

