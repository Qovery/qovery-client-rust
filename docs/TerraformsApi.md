# \TerraformsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clone_terraform**](TerraformsApi.md#clone_terraform) | **POST** /terraform/{terraformId}/clone | Clone terraform
[**create_terraform**](TerraformsApi.md#create_terraform) | **POST** /environment/{environmentId}/terraform | Create a terraform
[**get_default_terraform_advanced_settings**](TerraformsApi.md#get_default_terraform_advanced_settings) | **GET** /defaultTerraformAdvancedSettings: | List default terraform advanced settings
[**list_terraforms**](TerraformsApi.md#list_terraforms) | **GET** /environment/{environmentId}/terraform | List terraforms



## clone_terraform

> models::TerraformResponse clone_terraform(terraform_id, clone_service_request)
Clone terraform

This will create a new terraform with the same configuration on the targeted environment Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **uuid::Uuid** | Terraform ID | [required] |
**clone_service_request** | Option<[**CloneServiceRequest**](CloneServiceRequest.md)> |  |  |

### Return type

[**models::TerraformResponse**](TerraformResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_terraform

> models::TerraformResponse create_terraform(environment_id, terraform_request)
Create a terraform

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **String** |  | [required] |
**terraform_request** | Option<[**TerraformRequest**](TerraformRequest.md)> |  |  |

### Return type

[**models::TerraformResponse**](TerraformResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_terraform_advanced_settings

> models::TerraformAdvancedSettings get_default_terraform_advanced_settings()
List default terraform advanced settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TerraformAdvancedSettings**](TerraformAdvancedSettings.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_terraforms

> models::TerraformResponseList list_terraforms(environment_id)
List terraforms

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |

### Return type

[**models::TerraformResponseList**](TerraformResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

