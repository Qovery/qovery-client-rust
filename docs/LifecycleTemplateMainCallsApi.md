# \LifecycleTemplateMainCallsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_environment_lifecycle_template**](LifecycleTemplateMainCallsApi.md#get_environment_lifecycle_template) | **GET** /environment/{environmentId}/lifecycleTemplate/{lifecycleTemplateId} | Get specific lifecycle template
[**list_environment_lifecycle_templates**](LifecycleTemplateMainCallsApi.md#list_environment_lifecycle_templates) | **GET** /environment/{environmentId}/lifecycleTemplate | List available lifecycle template for this environment



## get_environment_lifecycle_template

> models::LifecycleTemplateResponse get_environment_lifecycle_template(environment_id, lifecycle_template_id)
Get specific lifecycle template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **String** |  | [required] |
**lifecycle_template_id** | **String** |  | [required] |

### Return type

[**models::LifecycleTemplateResponse**](LifecycleTemplateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_environment_lifecycle_templates

> models::LifecycleTemplateListResponse list_environment_lifecycle_templates(environment_id)
List available lifecycle template for this environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **String** |  | [required] |

### Return type

[**models::LifecycleTemplateListResponse**](LifecycleTemplateListResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

