# \ApplicationLogsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_application_log**](ApplicationLogsApi.md#list_application_log) | **GET** /application/{applicationId}/log | List logs



## list_application_log

> models::LogResponseList list_application_log(application_id)
List logs

This will list the last 1000 logs of the application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |

### Return type

[**models::LogResponseList**](LogResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

