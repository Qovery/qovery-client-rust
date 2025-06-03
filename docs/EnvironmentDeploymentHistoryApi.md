# \EnvironmentDeploymentHistoryApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_environment_deployment_history**](EnvironmentDeploymentHistoryApi.md#list_environment_deployment_history) | **GET** /environment/{environmentId}/deploymentHistory | List environment deployments
[**list_environment_deployment_history_v2**](EnvironmentDeploymentHistoryApi.md#list_environment_deployment_history_v2) | **GET** /environment/{environmentId}/deploymentHistoryV2 | List environment deployments



## list_environment_deployment_history

> models::DeploymentHistoryEnvironmentPaginatedResponseList list_environment_deployment_history(environment_id, start_id)
List environment deployments

List previous and current environment deployments with the status deployment and the related services. By default it returns the 20 last results. The response is paginated. In order to request the next page, you can use the startId query parameter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**start_id** | Option<**uuid::Uuid**> | Starting point after which to return results |  |

### Return type

[**models::DeploymentHistoryEnvironmentPaginatedResponseList**](DeploymentHistoryEnvironmentPaginatedResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_environment_deployment_history_v2

> models::DeploymentHistoryEnvironmentPaginatedResponseListV2 list_environment_deployment_history_v2(environment_id, start_id)
List environment deployments

List previous and current environment deployments with the status deployment and the related services. By default it returns the 20 last results. The response is paginated. In order to request the next page, you can use the startId query parameter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** |  | [required] |
**start_id** | Option<**uuid::Uuid**> | Starting point after which to return results |  |

### Return type

[**models::DeploymentHistoryEnvironmentPaginatedResponseListV2**](DeploymentHistoryEnvironmentPaginatedResponseListV2.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

