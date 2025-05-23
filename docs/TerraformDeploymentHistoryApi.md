# \TerraformDeploymentHistoryApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_terraform_deployment_history_v2**](TerraformDeploymentHistoryApi.md#list_terraform_deployment_history_v2) | **GET** /terraform/{terraformId}/deploymentHistoryV2 | List terraform deployments



## list_terraform_deployment_history_v2

> models::DeploymentHistoryServicePaginatedResponseListV2 list_terraform_deployment_history_v2(terraform_id)
List terraform deployments

Returns the 20 last terraform deployments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **uuid::Uuid** | Terraform ID | [required] |

### Return type

[**models::DeploymentHistoryServicePaginatedResponseListV2**](DeploymentHistoryServicePaginatedResponseListV2.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

