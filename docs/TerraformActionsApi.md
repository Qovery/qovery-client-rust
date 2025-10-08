# \TerraformActionsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**deploy_terraform**](TerraformActionsApi.md#deploy_terraform) | **POST** /terraform/{terraformId}/deploy | Deploy terraform
[**redeploy_terraform**](TerraformActionsApi.md#redeploy_terraform) | **POST** /terraform/{terraformId}/redeploy | Redeploy terraform
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


## redeploy_terraform

> models::Status redeploy_terraform(terraform_id)
Redeploy terraform

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **String** |  | [required] |

### Return type

[**models::Status**](Status.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uninstall_terraform

> models::Status uninstall_terraform(terraform_id, force_terraform_action, body)
Uninstall terraform

Delete the resources of the terraform but keep Qovery configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **uuid::Uuid** | Terraform ID | [required] |
**force_terraform_action** | Option<[**DeleteTerraformAction**](.md)> | Force a specific action to be executed by Terraform during uninstall. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::Status**](Status.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

