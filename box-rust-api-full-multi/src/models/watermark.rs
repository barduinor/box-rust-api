/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// Watermark : A watermark is a semi-transparent overlay on an embedded file preview that displays a viewer's email address or user ID and the time of access over a file's content



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Watermark {
    #[serde(rename = "watermark", skip_serializing_if = "Option::is_none")]
    pub watermark: Option<Box<crate::models::WatermarkWatermark>>,
}

impl Watermark {
    /// A watermark is a semi-transparent overlay on an embedded file preview that displays a viewer's email address or user ID and the time of access over a file's content
    pub fn new() -> Watermark {
        Watermark {
            watermark: None,
        }
    }
}


