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
pub struct ShieldInformationBarrierSegmentRestrictionMiniAllOf {
    #[serde(rename = "shield_information_barrier_segment", skip_serializing_if = "Option::is_none")]
    pub shield_information_barrier_segment: Option<Box<crate::models::ShieldInformationBarrierSegmentRestrictionMiniAllOfShieldInformationBarrierSegment>>,
    #[serde(rename = "restricted_segment", skip_serializing_if = "Option::is_none")]
    pub restricted_segment: Option<Box<crate::models::ShieldInformationBarrierSegmentRestrictionMiniAllOfRestrictedSegment>>,
}

impl ShieldInformationBarrierSegmentRestrictionMiniAllOf {
    pub fn new() -> ShieldInformationBarrierSegmentRestrictionMiniAllOf {
        ShieldInformationBarrierSegmentRestrictionMiniAllOf {
            shield_information_barrier_segment: None,
            restricted_segment: None,
        }
    }
}


