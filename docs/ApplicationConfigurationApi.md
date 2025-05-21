# \ApplicationConfigurationApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**edit_advanced_settings**](ApplicationConfigurationApi.md#edit_advanced_settings) | **PUT** /application/{applicationId}/advancedSettings | Edit advanced settings
[**edit_application_network**](ApplicationConfigurationApi.md#edit_application_network) | **PUT** /application/{applicationId}/network | Edit Application Network
[**get_advanced_settings**](ApplicationConfigurationApi.md#get_advanced_settings) | **GET** /application/{applicationId}/advancedSettings | Get advanced settings
[**get_application_network**](ApplicationConfigurationApi.md#get_application_network) | **GET** /application/{applicationId}/network | Get Application Network information



## edit_advanced_settings

> models::ApplicationAdvancedSettings edit_advanced_settings(application_id, application_advanced_settings)
Edit advanced settings

Edit advanced settings by returning table of advanced settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |
**application_advanced_settings** | Option<[**ApplicationAdvancedSettings**](ApplicationAdvancedSettings.md)> |  |  |

### Return type

[**models::ApplicationAdvancedSettings**](ApplicationAdvancedSettings.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_application_network

> models::ApplicationNetwork edit_application_network(application_id, application_network_request)
Edit Application Network

Edit the Network settings of the application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |
**application_network_request** | Option<[**ApplicationNetworkRequest**](ApplicationNetworkRequest.md)> |  |  |

### Return type

[**models::ApplicationNetwork**](ApplicationNetwork.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_advanced_settings

> models::ApplicationAdvancedSettings get_advanced_settings(application_id)
Get advanced settings

Get list and values of the advanced settings of the application. Default values for each setting are available in [our documentation](https://hub.qovery.com/docs/using-qovery/configuration/advanced-settings/) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |

### Return type

[**models::ApplicationAdvancedSettings**](ApplicationAdvancedSettings.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_application_network

> models::ApplicationNetwork get_application_network(application_id)
Get Application Network information

Get status of the application network settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |

### Return type

[**models::ApplicationNetwork**](ApplicationNetwork.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

