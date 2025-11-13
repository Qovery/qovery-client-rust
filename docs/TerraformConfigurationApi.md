# \TerraformConfigurationApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_terraform_variable**](TerraformConfigurationApi.md#delete_terraform_variable) | **DELETE** /terraform/{terraformId}/variables/{key} | Delete a terraform variable
[**edit_terraform_advanced_settings**](TerraformConfigurationApi.md#edit_terraform_advanced_settings) | **PUT** /terraform/{terraformId}/advancedSettings | Edit Advanced settings
[**get_terraform_advanced_settings**](TerraformConfigurationApi.md#get_terraform_advanced_settings) | **GET** /terraform/{terraformId}/advancedSettings | Get Advanced settings
[**get_terraform_variables**](TerraformConfigurationApi.md#get_terraform_variables) | **GET** /terraform/{terraformId}/variables | Get terraform variables
[**replace_all_terraform_variables**](TerraformConfigurationApi.md#replace_all_terraform_variables) | **PUT** /terraform/{terraformId}/variables | Replace all terraform variables



## delete_terraform_variable

> delete_terraform_variable(terraform_id, key)
Delete a terraform variable

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **uuid::Uuid** | Terraform ID | [required] |
**key** | **String** | Variable key to delete | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_terraform_advanced_settings

> models::TerraformAdvancedSettings edit_terraform_advanced_settings(terraform_id, terraform_advanced_settings)
Edit Advanced settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **uuid::Uuid** |  | [required] |
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
**terraform_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::TerraformAdvancedSettings**](TerraformAdvancedSettings.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_terraform_variables

> models::TerraformVariablesResponse get_terraform_variables(terraform_id)
Get terraform variables

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **uuid::Uuid** | Terraform ID | [required] |

### Return type

[**models::TerraformVariablesResponse**](TerraformVariablesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_all_terraform_variables

> replace_all_terraform_variables(terraform_id, terraform_variables_replace_request)
Replace all terraform variables

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **uuid::Uuid** | Terraform ID | [required] |
**terraform_variables_replace_request** | [**TerraformVariablesReplaceRequest**](TerraformVariablesReplaceRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

