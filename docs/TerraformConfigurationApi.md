# \TerraformConfigurationApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**edit_terraform_advanced_settings**](TerraformConfigurationApi.md#edit_terraform_advanced_settings) | **PUT** /terraform/{terraformId}/advancedSettings | Edit Advanced settings
[**get_terraform_advanced_settings**](TerraformConfigurationApi.md#get_terraform_advanced_settings) | **GET** /terraform/{terraformId}/advancedSettings | Get Advanced settings



## edit_terraform_advanced_settings

> models::TerraformAdvancedSettings edit_terraform_advanced_settings(terraform_id, terraform_advanced_settings)
Edit Advanced settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **uuid::Uuid** | Terraform ID | [required] |
**terraform_advanced_settings** | Option<[**TerraformAdvancedSettings**](TerraformAdvancedSettings.md)> |  |  |

### Return type

[**models::TerraformAdvancedSettings**](TerraformAdvancedSettings.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_terraform_advanced_settings

> models::TerraformAdvancedSettings get_terraform_advanced_settings(terraform_id)
Get Advanced settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **uuid::Uuid** | Terraform ID | [required] |

### Return type

[**models::TerraformAdvancedSettings**](TerraformAdvancedSettings.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

