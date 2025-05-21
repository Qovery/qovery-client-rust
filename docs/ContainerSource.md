# ContainerSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**image_name** | **String** | The image name pattern differs according to chosen container registry provider: * `ECR`: `repository` * `SCALEWAY_CR`: `namespace/image` * `DOCKER_HUB`: `image` or `repository/image` * `PUBLIC_ECR`: `registry_alias/repository`  | 
**tag** | **String** | tag of the image container | 
**registry_id** | Option<**String**> | tag of the image container | [optional]
**registry** | [**models::ContainerRegistryProviderDetailsResponse**](ContainerRegistryProviderDetailsResponse.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


