# \ApplicationDeploymentHistoryApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_application_deployment_history**](ApplicationDeploymentHistoryApi.md#list_application_deployment_history) | **GET** /application/{applicationId}/deploymentHistory | List application deploys
[**list_application_deployment_history_v2**](ApplicationDeploymentHistoryApi.md#list_application_deployment_history_v2) | **GET** /application/{applicationId}/deploymentHistoryV2 | List application deploys



## list_application_deployment_history

> models::DeploymentHistoryPaginatedResponseList list_application_deployment_history(application_id, start_id)
List application deploys

By default it returns the 20 last results. The response is paginated. In order to request the next page, you can use the startId query parameter. You can also filter by status (FAILED or SUCCESS), and git_commit_id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |
**start_id** | Option<**uuid::Uuid**> | Starting point after which to return results |  |

### Return type

[**models::DeploymentHistoryPaginatedResponseList**](DeploymentHistoryPaginatedResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_application_deployment_history_v2

> models::DeploymentHistoryServicePaginatedResponseListV2 list_application_deployment_history_v2(application_id, page_size)
List application deploys

By default it returns the 20 last results. The response is paginated. In order to request the next page, you can use the startId query parameter. You can also filter by status (FAILED or SUCCESS), and git_commit_id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**page_size** | Option<**f64**> | The number of deployments to return in the current page |  |[default to 20]

### Return type

[**models::DeploymentHistoryServicePaginatedResponseListV2**](DeploymentHistoryServicePaginatedResponseListV2.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

