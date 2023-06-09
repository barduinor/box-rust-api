/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// LegalHoldPolicyMini : A mini legal hold policy



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LegalHoldPolicyMini {
    /// The unique identifier for this legal hold policy
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// `legal_hold_policy`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
}

impl LegalHoldPolicyMini {
    /// A mini legal hold policy
    pub fn new() -> LegalHoldPolicyMini {
        LegalHoldPolicyMini {
            id: None,
            r#type: None,
        }
    }
}

/// `legal_hold_policy`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "legal_hold_policy")]
    LegalHoldPolicy,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::LegalHoldPolicy
    }
}

