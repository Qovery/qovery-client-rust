# ContainerRegistryResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**kind** | Option<[**models::ContainerRegistryKindEnum**](ContainerRegistryKindEnum.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**url** | Option<**String**> | URL of the container registry | [optional]
**cluster** | Option<[**models::ContainerRegistryResponseAllOfCluster**](ContainerRegistryResponse_allOf_cluster.md)> |  | [optional]
**associated_services_count** | Option<**i32**> | The number of services using this container registry | [optional]
**config** | Option<[**models::ContainerRegistryResponseAllOfConfig**](ContainerRegistryResponse_allOf_config.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


