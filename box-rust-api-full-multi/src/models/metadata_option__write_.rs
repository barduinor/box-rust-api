/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// MetadataOptionWrite : An option for a Metadata Template Field.  Options only need to be provided for fields of type `enum` and `multiSelect`. Options represent the value(s) a user can select for the field either through the UI or through the API.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetadataOptionWrite {
    /// The text value of the option. This represents both the display name of the option and the internal key used when updating templates.
    #[serde(rename = "key")]
    pub key: String,
}

impl MetadataOptionWrite {
    /// An option for a Metadata Template Field.  Options only need to be provided for fields of type `enum` and `multiSelect`. Options represent the value(s) a user can select for the field either through the UI or through the API.
    pub fn new(key: String) -> MetadataOptionWrite {
        MetadataOptionWrite {
            key,
        }
    }
}


