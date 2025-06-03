# \TerraformDeploymentRestrictionApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_terraform_deployment_restriction**](TerraformDeploymentRestrictionApi.md#create_terraform_deployment_restriction) | **POST** /terraform/{terraformId}/deploymentRestriction | Create a terraform deployment restriction
[**delete_terraform_deployment_restriction**](TerraformDeploymentRestrictionApi.md#delete_terraform_deployment_restriction) | **DELETE** /terraform/{terraformId}/deploymentRestriction/{deploymentRestrictionId} | Delete a terraform deployment restriction
[**edit_terraform_deployment_restriction**](TerraformDeploymentRestrictionApi.md#edit_terraform_deployment_restriction) | **PUT** /terraform/{terraformId}/deploymentRestriction/{deploymentRestrictionId} | Edit a terraform deployment restriction
[**get_terraform_deployment_restrictions**](TerraformDeploymentRestrictionApi.md#get_terraform_deployment_restrictions) | **GET** /terraform/{terraformId}/deploymentRestriction | Get terraform deployment restrictions



## create_terraform_deployment_restriction

> models::TerraformDeploymentRestrictionResponse create_terraform_deployment_restriction(terraform_id, terraform_deployment_restriction_request)
Create a terraform deployment restriction

Create a terraform deployment restriction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **String** |  | [required] |
**terraform_deployment_restriction_request** | Option<[**TerraformDeploymentRestrictionRequest**](TerraformDeploymentRestrictionRequest.md)> |  |  |

### Return type

[**models::TerraformDeploymentRestrictionResponse**](TerraformDeploymentRestrictionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_terraform_deployment_restriction

> delete_terraform_deployment_restriction(terraform_id, deployment_restriction_id)
Delete a terraform deployment restriction

Delete a terraform deployment restriction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **uuid::Uuid** |  | [required] |
**deployment_restriction_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_terraform_deployment_restriction

> models::TerraformDeploymentRestrictionResponse edit_terraform_deployment_restriction(terraform_id, deployment_restriction_id, terraform_deployment_restriction_request)
Edit a terraform deployment restriction

Edit a terraform deployment restriction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **uuid::Uuid** |  | [required] |
**deployment_restriction_id** | **uuid::Uuid** |  | [required] |
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
**terraform_id** | **String** |  | [required] |

### Return type

[**models::TerraformDeploymentRestrictionResponseList**](TerraformDeploymentRestrictionResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

