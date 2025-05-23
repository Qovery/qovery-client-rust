# \TerraformDeploymentRestrictionApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_terraform_deployment_restriction**](TerraformDeploymentRestrictionApi.md#create_terraform_deployment_restriction) | **POST** /terraform/{terraformId}/deploymentRestriction | Create a terraform deployment restriction
[**get_terraform_deployment_restrictions**](TerraformDeploymentRestrictionApi.md#get_terraform_deployment_restrictions) | **GET** /terraform/{terraformId}/deploymentRestriction | Get terraform deployment restrictions



## create_terraform_deployment_restriction

> models::TerraformDeploymentRestrictionResponse create_terraform_deployment_restriction(terraform_id, terraform_deployment_restriction_request)
Create a terraform deployment restriction

Create a terraform deployment restriction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **uuid::Uuid** | Terraform ID | [required] |
**terraform_deployment_restriction_request** | Option<[**TerraformDeploymentRestrictionRequest**](TerraformDeploymentRestrictionRequest.md)> |  |  |

### Return type

[**models::TerraformDeploymentRestrictionResponse**](TerraformDeploymentRestrictionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_terraform_deployment_restrictions

> models::TerraformDeploymentRestrictionResponseList get_terraform_deployment_restrictions(terraform_id)
Get terraform deployment restrictions

Get terraform deployment restrictions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **uuid::Uuid** | Terraform ID | [required] |

### Return type

[**models::TerraformDeploymentRestrictionResponseList**](TerraformDeploymentRestrictionResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

