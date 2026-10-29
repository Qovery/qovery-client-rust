# BlueprintManifestFieldType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** |  (enum: string, bool, number) | 
**pattern** | Option<**String**> | Regex pattern, only present for string fields | [optional]
**min_length** | Option<**i64**> | Minimum length, only present for string fields | [optional]
**max_length** | Option<**i64**> | Maximum length, only present for string fields | [optional]
**format** | Option<**String**> | Optional editor format, only present for string fields that declare one. json selects a JSON editor; the value is still sent and stored as a string, and the API rejects text that is not one valid JSON document. Unknown formats should fall back to the ordinary string editor. | [optional]
**min** | Option<**i64**> | Lower bound, only present for number fields | [optional]
**max** | Option<**i64**> | Upper bound, only present for number fields | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


