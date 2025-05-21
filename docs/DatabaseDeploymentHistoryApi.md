# \DatabaseDeploymentHistoryApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_database_deployment_history**](DatabaseDeploymentHistoryApi.md#list_database_deployment_history) | **GET** /database/{databaseId}/deploymentHistory | List database deploys
[**list_database_deployment_history_v2**](DatabaseDeploymentHistoryApi.md#list_database_deployment_history_v2) | **GET** /database/{databaseId}/deploymentHistoryV2 | List database deploys



## list_database_deployment_history

> models::ListDatabaseDeploymentHistory200Response list_database_deployment_history(database_id, start_id)
List database deploys

By default it returns the 20 last results. The response is paginated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **uuid::Uuid** | Database ID | [required] |
**start_id** | Option<**uuid::Uuid**> | Starting point after which to return results |  |

### Return type

[**models::ListDatabaseDeploymentHistory200Response**](listDatabaseDeploymentHistory_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_database_deployment_history_v2

> models::DeploymentHistoryServicePaginatedResponseListV2 list_database_deployment_history_v2(database_id, start_id)
List database deploys

By default it returns the 20 last results. The response is paginated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **uuid::Uuid** | Database ID | [required] |
**start_id** | Option<**uuid::Uuid**> | Starting point after which to return results |  |

### Return type

[**models::DeploymentHistoryServicePaginatedResponseListV2**](DeploymentHistoryServicePaginatedResponseListV2.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

