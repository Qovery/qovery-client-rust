# \TerraformDeploymentHistoryApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_terraform_deployment_history_v2**](TerraformDeploymentHistoryApi.md#list_terraform_deployment_history_v2) | **GET** /terraform/{terraformId}/deploymentHistoryV2 | List terraform deployments



## list_terraform_deployment_history_v2

> models::DeploymentHistoryServicePaginatedResponseListV2 list_terraform_deployment_history_v2(terraform_id, page_size)
List terraform deployments

Returns the 20 last terraform deployments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **uuid::Uuid** |  | [required] |
**page_size** | Option<**f64**> | The number of deployments to return in the current page |  |[default to 20]

### Return type

[**models::DeploymentHistoryServicePaginatedResponseListV2**](DeploymentHistoryServicePaginatedResponseListV2.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

