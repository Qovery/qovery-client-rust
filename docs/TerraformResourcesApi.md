# \TerraformResourcesApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_terraform_resources**](TerraformResourcesApi.md#get_terraform_resources) | **GET** /api/v1/terraform/{terraformId}/terraformResources | Get terraform resources from latest deployment



## get_terraform_resources

> models::TerraformResourcesResponse get_terraform_resources(terraform_id)
Get terraform resources from latest deployment

Returns the list of Terraform resources from the most recent deployment execution

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **uuid::Uuid** | Terraform ID | [required] |

### Return type

[**models::TerraformResourcesResponse**](TerraformResourcesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

