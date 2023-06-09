# GetShieldInformationBarrierSegments200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**limit** | Option<**i64**> | The limit that was used for these entries. This will be the same as the `limit` query parameter unless that value exceeded the maximum value allowed. The maximum value varies by API. | [optional]
**next_marker** | Option<**String**> | The marker for the start of the next page of results. | [optional]
**entries** | Option<[**Vec<crate::models::ShieldInformationBarrierSegment>**](ShieldInformationBarrierSegment.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

