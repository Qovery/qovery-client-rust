# HelmRepositoryResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | **String** |  | 
**kind** | Option<[**models::HelmRepositoryKindEnum**](HelmRepositoryKindEnum.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**url** | Option<**String**> | URL of the helm repository | [optional]
**skip_tls_verification** | Option<**bool**> | Bypass tls certificate verification when connecting to repository | [optional]
**associated_services_count** | Option<**i32**> | The number of services using this helm repository | [optional]
**config** | Option<[**models::HelmRepositoryResponseAllOfConfig**](HelmRepositoryResponse_allOf_config.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


