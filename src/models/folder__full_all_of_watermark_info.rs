/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FolderFullAllOfWatermarkInfo {
    /// Specifies if this item has a watermark applied.
    #[serde(rename = "is_watermarked", skip_serializing_if = "Option::is_none")]
    pub is_watermarked: Option<bool>,
}

impl FolderFullAllOfWatermarkInfo {
    pub fn new() -> FolderFullAllOfWatermarkInfo {
        FolderFullAllOfWatermarkInfo {
            is_watermarked: None,
        }
    }
}

