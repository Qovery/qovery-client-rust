# \ServiceMainCallsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_service_git_webhook_status**](ServiceMainCallsApi.md#get_service_git_webhook_status) | **GET** /service/{serviceId}/gitWebhookStatus | Get git webhook status for a service



## get_service_git_webhook_status

> models::GitWebhookStatusResponse get_service_git_webhook_status(service_id)
Get git webhook status for a service

Returns the webhook status for a git-based service. Checks if the Qovery webhook is correctly configured on the git provider (GitHub, GitLab, or Bitbucket).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **uuid::Uuid** | Service ID of an application/job/container/database | [required] |

### Return type

[**models::GitWebhookStatusResponse**](GitWebhookStatusResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

