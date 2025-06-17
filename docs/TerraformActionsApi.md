# \TerraformActionsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**deploy_terraform**](TerraformActionsApi.md#deploy_terraform) | **POST** /terraform/{terraformId}/deploy | Deploy terraform
[**uninstall_terraform**](TerraformActionsApi.md#uninstall_terraform) | **POST** /terraform/{terraformId}/uninstall | Uninstall terraform



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


## uninstall_terraform

> serde_json::Value uninstall_terraform(terraform_id, body)
Uninstall terraform

Delete the resources of the terraform but keep Qovery configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **uuid::Uuid** | Terraform ID | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

