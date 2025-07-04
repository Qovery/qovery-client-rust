# \ContainerDeploymentHistoryApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_container_deployment_history**](ContainerDeploymentHistoryApi.md#list_container_deployment_history) | **GET** /container/{containerId}/deploymentHistory | List container deployments
[**list_container_deployment_history_v2**](ContainerDeploymentHistoryApi.md#list_container_deployment_history_v2) | **GET** /container/{containerId}/deploymentHistoryV2 | List container deployments



## list_container_deployment_history

> models::ListContainerDeploymentHistory200Response list_container_deployment_history(container_id)
List container deployments

Returns the 20 last container deployments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** | Container ID | [required] |

### Return type

[**models::ListContainerDeploymentHistory200Response**](listContainerDeploymentHistory_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_container_deployment_history_v2

> models::DeploymentHistoryServicePaginatedResponseListV2 list_container_deployment_history_v2(container_id, page_size)
List container deployments

Returns the 20 last container deployments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** | Container ID | [required] |
**page_size** | Option<**f64**> | The number of deployments to return in the current page |  |[default to 20]

### Return type

[**models::DeploymentHistoryServicePaginatedResponseListV2**](DeploymentHistoryServicePaginatedResponseListV2.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

