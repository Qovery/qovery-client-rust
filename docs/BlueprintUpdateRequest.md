# BlueprintUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Display name for the service | 
**tag** | **String** | Catalog tag identifying the target blueprint version | 
**icon** | **String** | Icon URL for the service | 
**variables** | Option<[**std::collections::HashMap<String, models::BlueprintUpdateVariableValue>**](BlueprintUpdateVariableValue.md)> | RFC 7396 patch map keyed by variable name. Non-null value upserts the variable; null value removes it. Absent keys are left untouched. Omitting the field entirely is equivalent to an empty map — no variables are modified. | [optional]
**spec_overrides** | Option<**std::collections::HashMap<String, serde_json::Value>**> | JSON Merge Patch (RFC 7396) applied to the stored spec_overrides. Keys with a non-null value are upserted; keys with a null value are removed. Pass null or omit the field to leave all existing overrides unchanged. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


