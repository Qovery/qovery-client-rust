# \ContainerLogsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_container_log**](ContainerLogsApi.md#list_container_log) | **GET** /container/{containerId}/log | List logs



## list_container_log

> models::LogResponseList list_container_log(container_id)
List logs

This will list the last 1000 logs of the container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** | Container ID | [required] |

### Return type

[**models::LogResponseList**](LogResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

