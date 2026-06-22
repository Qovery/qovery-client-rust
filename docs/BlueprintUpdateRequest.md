# BlueprintUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Display name for the service | 
**tag** | **String** | Catalog tag identifying the target blueprint version | 
**icon** | **String** | Icon URL for the service | 
**variables** | Option<[**std::collections::HashMap<String, models::BlueprintUpdateVariableValue>**](BlueprintUpdateVariableValue.md)> | RFC 7396 patch map keyed by variable name. Non-null value upserts the variable; null value removes it. Absent keys are left untouched. Omitting the field entirely is equivalent to an empty map — no variables are modified. | [optional]
**spec_overrides** | Option<[**models::BlueprintSpecOverrides**](BlueprintSpecOverrides.md)> | JSON Merge Patch (RFC 7396) applied to the stored spec_overrides (see `BlueprintSpecOverrides` for the list of valid fields). A non-null field value upserts the override; a null value removes it. Pass null or omit the field entirely to leave all existing overrides unchanged. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


