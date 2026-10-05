# BlueprintDatabaseResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [**models::DatabaseTypeEnum**](DatabaseTypeEnum.md) |  | 
**service_id** | Option<**uuid::Uuid**> | Terraform service the blueprint created; null before its first dispatch | 
**endpoint** | Option<[**models::DatabaseEndpointsResponse**](DatabaseEndpointsResponse.md)> | Null until a deploy has reported the database endpoint | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


