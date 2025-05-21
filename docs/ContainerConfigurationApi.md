# \ContainerConfigurationApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**edit_container_advanced_settings**](ContainerConfigurationApi.md#edit_container_advanced_settings) | **PUT** /container/{containerId}/advancedSettings | Edit advanced settings
[**edit_container_network**](ContainerConfigurationApi.md#edit_container_network) | **PUT** /container/{containerId}/network | Edit Container Network
[**get_container_advanced_settings**](ContainerConfigurationApi.md#get_container_advanced_settings) | **GET** /container/{containerId}/advancedSettings | Get advanced settings
[**get_container_network**](ContainerConfigurationApi.md#get_container_network) | **GET** /container/{containerId}/network | Get Container Network information



## edit_container_advanced_settings

> models::ContainerAdvancedSettings edit_container_advanced_settings(container_id, container_advanced_settings)
Edit advanced settings

Edit advanced settings by returning table of advanced settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** | Container ID | [required] |
**container_advanced_settings** | Option<[**ContainerAdvancedSettings**](ContainerAdvancedSettings.md)> |  |  |

### Return type

[**models::ContainerAdvancedSettings**](ContainerAdvancedSettings.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_container_network

> models::ContainerNetwork edit_container_network(container_id, container_network_request)
Edit Container Network

Edit the Network settings of the container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** | Container ID | [required] |
**container_network_request** | Option<[**ContainerNetworkRequest**](ContainerNetworkRequest.md)> |  |  |

### Return type

[**models::ContainerNetwork**](ContainerNetwork.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_container_advanced_settings

> models::ContainerAdvancedSettings get_container_advanced_settings(container_id)
Get advanced settings

Get list and values of the advanced settings of the container. Default values for each setting are available in [our documentation](https://hub.qovery.com/docs/using-qovery/configuration/advanced-settings/) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** | Container ID | [required] |

### Return type

[**models::ContainerAdvancedSettings**](ContainerAdvancedSettings.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_container_network

> models::ContainerNetwork get_container_network(container_id)
Get Container Network information

Get status of the container network settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** | Container ID | [required] |

### Return type

[**models::ContainerNetwork**](ContainerNetwork.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

