# GitTokenResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**r#type** | [**models::GitProviderEnum**](GitProviderEnum.md) |  | 
**expired_at** | Option<[**String**](string.md)> |  | [optional]
**workspace** | Option<**String**> | Mandatory only for BITBUCKET git provider | [optional]
**associated_services_count** | **f64** | The number of services using this git token | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


