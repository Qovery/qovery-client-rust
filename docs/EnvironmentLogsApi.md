# \EnvironmentLogsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_environment_logs**](EnvironmentLogsApi.md#list_environment_logs) | **GET** /environment/{environmentId}/logs | List environment deployment logs v2



## list_environment_logs

> Vec<models::EnvironmentLogs> list_environment_logs(environment_id, version)
List environment deployment logs v2

This returns the last 1000 environment deployment logs v2

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**version** | Option<**String**> |  |  |

### Return type

[**Vec<models::EnvironmentLogs>**](EnvironmentLogs.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

