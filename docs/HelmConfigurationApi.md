# \HelmConfigurationApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**edit_helm_advanced_settings**](HelmConfigurationApi.md#edit_helm_advanced_settings) | **PUT** /helm/{helmId}/advancedSettings | Edit advanced settings
[**get_helm_advanced_settings**](HelmConfigurationApi.md#get_helm_advanced_settings) | **GET** /helm/{helmId}/advancedSettings | Get advanced settings



## edit_helm_advanced_settings

> models::HelmAdvancedSettings edit_helm_advanced_settings(helm_id, helm_advanced_settings)
Edit advanced settings

Edit advanced settings by returning table of advanced settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**helm_id** | **uuid::Uuid** | Helm ID | [required] |
**helm_advanced_settings** | Option<[**HelmAdvancedSettings**](HelmAdvancedSettings.md)> |  |  |

### Return type

[**models::HelmAdvancedSettings**](HelmAdvancedSettings.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_helm_advanced_settings

> models::HelmAdvancedSettings get_helm_advanced_settings(helm_id)
Get advanced settings

Get list and values of the advanced settings of the helm.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**helm_id** | **uuid::Uuid** | Helm ID | [required] |

### Return type

[**models::HelmAdvancedSettings**](HelmAdvancedSettings.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

