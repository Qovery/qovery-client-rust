# BlueprintUpdatePreviewRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Display name for the service | 
**tag** | **String** | Catalog tag identifying the target blueprint version | 
**icon** | **String** | Icon URL for the service | 
**variables** | Option<[**Vec<models::BlueprintVariableRequest>**](BlueprintVariableRequest.md)> | Variable overrides to apply in the preview | [optional]
**spec_overrides** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Partial spec overrides merged on top of the blueprint manifest | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


