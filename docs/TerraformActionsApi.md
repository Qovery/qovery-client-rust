# \TerraformActionsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**deploy_terraform**](TerraformActionsApi.md#deploy_terraform) | **POST** /terraform/{terraformId}/deploy | Deploy terraform



## deploy_terraform

> models::Status deploy_terraform(terraform_id, terraform_deploy_request)
Deploy terraform

You must provide a git commit id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **uuid::Uuid** | Terraform ID | [required] |
**terraform_deploy_request** | Option<[**TerraformDeployRequest**](TerraformDeployRequest.md)> |  |  |

### Return type

[**models::Status**](Status.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

