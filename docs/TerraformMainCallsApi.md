# \TerraformMainCallsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_terraform**](TerraformMainCallsApi.md#delete_terraform) | **DELETE** /terraform/{terraformId} | Delete Terraform
[**edit_terraform**](TerraformMainCallsApi.md#edit_terraform) | **PUT** /terraform/{terraformId} | Edit Terraform
[**get_terraform**](TerraformMainCallsApi.md#get_terraform) | **GET** /terraform/{terraformId} | Get terraform by ID
[**list_terraform_commit**](TerraformMainCallsApi.md#list_terraform_commit) | **GET** /terraform/{terraformId}/commit | List last commits
[**list_terraform_versions**](TerraformMainCallsApi.md#list_terraform_versions) | **GET** /terraform/availableVersion | List available Terraform versions



## delete_terraform

> delete_terraform(terraform_id, resources_only, force_terraform_action)
Delete Terraform

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **uuid::Uuid** | Terraform ID | [required] |
**resources_only** | Option<**bool**> | When true, only resources are deleted and Qovery configuration is kept. |  |[default to false]
**force_terraform_action** | Option<[**DeleteTerraformAction**](.md)> | Force a specific action to be executed by Terraform during deletion. |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_terraform

> models::TerraformResponse edit_terraform(terraform_id, terraform_request)
Edit Terraform

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **uuid::Uuid** | Terraform ID | [required] |
**terraform_request** | Option<[**TerraformRequest**](TerraformRequest.md)> |  |  |

### Return type

[**models::TerraformResponse**](TerraformResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_terraform

> models::TerraformResponse get_terraform(terraform_id)
Get terraform by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **uuid::Uuid** | Terraform ID | [required] |

### Return type

[**models::TerraformResponse**](TerraformResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_terraform_commit

> models::CommitResponseList list_terraform_commit(terraform_id)
List last commits

Returns list of the last 100 commits made on the repository linked to the terraform service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terraform_id** | **uuid::Uuid** | Terraform ID | [required] |

### Return type

[**models::CommitResponseList**](CommitResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_terraform_versions

> models::TerraformVersionResponseList list_terraform_versions()
List available Terraform versions

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TerraformVersionResponseList**](TerraformVersionResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

