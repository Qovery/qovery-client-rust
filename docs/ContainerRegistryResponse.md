# ContainerRegistryResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**kind** | Option<[**models::ContainerRegistryKindEnum**](ContainerRegistryKindEnum.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**url** | Option<**String**> | URL of the container registry | [optional]
**cluster** | Option<[**models::ContainerRegistryResponseAllOfCluster**](ContainerRegistryResponseAllOfCluster.md)> |  | [optional]
**associated_services_count** | Option<**i32**> | The number of services using this container registry | [optional]
**config** | Option<[**models::ContainerRegistryResponseAllOfConfig**](ContainerRegistryResponseAllOfConfig.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


