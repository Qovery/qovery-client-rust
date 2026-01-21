# EnvironmentOverviewResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**name** | **String** |  | 
**mode** | [**models::EnvironmentModeEnum**](EnvironmentModeEnum.md) |  | 
**cluster** | Option<[**models::ClusterOverviewResponse**](ClusterOverviewResponse.md)> |  | [optional]
**service_count** | **i32** |  | 
**deployment_status** | Option<[**models::EnvironmentStatus**](EnvironmentStatus.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


