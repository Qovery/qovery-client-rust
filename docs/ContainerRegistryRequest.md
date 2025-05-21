# ContainerRegistryRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**kind** | [**models::ContainerRegistryKindEnum**](ContainerRegistryKindEnum.md) |  | 
**description** | Option<**String**> |  | [optional]
**url** | Option<**String**> | URL of the container registry: * For `DOCKER_HUB`: it must be `https://docker.io` (default with 'https://docker.io' if no url provided for `DOCKER_HUB`) * For `GITHUB_CR`: it must be `https://ghcr.io` (default with 'https://ghcr.io' if no url provided for `GITHUB_CR`) * For `GITLAB_CR`: it must be `https://registry.gitlab.com` (default with 'https://registry.gitlab.com' if no url provided for `GITLAB_CR`) * For others: it's required and must start by `https://`  | [optional]
**config** | [**models::ContainerRegistryRequestConfig**](ContainerRegistryRequest_config.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


