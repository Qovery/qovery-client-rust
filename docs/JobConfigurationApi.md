# \JobConfigurationApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**edit_job_advanced_settings**](JobConfigurationApi.md#edit_job_advanced_settings) | **PUT** /job/{jobId}/advancedSettings | Edit advanced settings
[**get_job_advanced_settings**](JobConfigurationApi.md#get_job_advanced_settings) | **GET** /job/{jobId}/advancedSettings | Get advanced settings



## edit_job_advanced_settings

> models::JobAdvancedSettings edit_job_advanced_settings(job_id, job_advanced_settings)
Edit advanced settings

Edit advanced settings by returning table of advanced settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |
**job_advanced_settings** | Option<[**JobAdvancedSettings**](JobAdvancedSettings.md)> |  |  |

### Return type

[**models::JobAdvancedSettings**](JobAdvancedSettings.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_advanced_settings

> models::JobAdvancedSettings get_job_advanced_settings(job_id)
Get advanced settings

Get list and values of the advanced settings of the job. Default values for each setting are available in [our documentation](https://hub.qovery.com/docs/using-qovery/configuration/advanced-settings/) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |

### Return type

[**models::JobAdvancedSettings**](JobAdvancedSettings.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

