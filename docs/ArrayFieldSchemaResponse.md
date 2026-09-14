# ArrayFieldSchemaResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** |  | 
**r#type** | **Type** |  (enum: array) | 
**required** | **bool** |  | 
**label** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**sensitive** | **bool** |  | 
**constraints** | [**models::CollectionConstraintsResponse**](CollectionConstraintsResponse.md) |  | 
**items** | [**models::ArrayItemResponse**](ArrayItemResponse.md) |  | 
**item_fields** | Option<[**Vec<Vec<models::FieldSchemaResponse>>**](Vec.md)> | Evaluated field descriptors for each object item, in the same order as the configuration array. Use these row-specific descriptors when available; items.fields describes an object's fields before per-item evaluation. Omitted when unavailable, including scalar arrays. An evaluated empty object array has an empty itemFields array. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


