# \DeploymentQueueActionsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_deployment_request**](DeploymentQueueActionsApi.md#cancel_deployment_request) | **POST** /deploymentQueue/{deploymentRequestId}/cancelDeployment | Cancel deployment request



## cancel_deployment_request

> cancel_deployment_request(deployment_request_id, body)
Cancel deployment request

Cancel the a deployment request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_request_id** | **uuid::Uuid** |  | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

