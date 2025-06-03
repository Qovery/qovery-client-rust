# \HelmsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clone_helm**](HelmsApi.md#clone_helm) | **POST** /helm/{helmId}/clone | Clone helm
[**create_helm**](HelmsApi.md#create_helm) | **POST** /environment/{environmentId}/helm | Create a helm
[**create_helm_default_values**](HelmsApi.md#create_helm_default_values) | **POST** /environment/{environmentId}/helmDefaultValues | Get helm default values
[**get_default_helm_advanced_settings**](HelmsApi.md#get_default_helm_advanced_settings) | **GET** /defaultHelmAdvancedSettings | List default helm advanced settings
[**get_environment_helm_status**](HelmsApi.md#get_environment_helm_status) | **GET** /environment/{environmentId}/helm/status | List all environment helm statuses
[**list_helms**](HelmsApi.md#list_helms) | **GET** /environment/{environmentId}/helm | List helms



## clone_helm

> models::HelmResponse clone_helm(helm_id, clone_service_request)
Clone helm

This will create a new helm with the same configuration on the targeted environment Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**helm_id** | **uuid::Uuid** | Helm ID | [required] |
**clone_service_request** | Option<[**CloneServiceRequest**](CloneServiceRequest.md)> |  |  |

### Return type

[**models::HelmResponse**](HelmResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_helm

> models::HelmResponse create_helm(environment_id, helm_request)
Create a helm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**helm_request** | Option<[**HelmRequest**](HelmRequest.md)> |  |  |

### Return type

[**models::HelmResponse**](HelmResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_helm_default_values

> String create_helm_default_values(environment_id, helm_default_values_request)
Get helm default values

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**helm_default_values_request** | Option<[**HelmDefaultValuesRequest**](HelmDefaultValuesRequest.md)> |  |  |

### Return type

**String**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_helm_advanced_settings

> models::HelmAdvancedSettings get_default_helm_advanced_settings()
List default helm advanced settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HelmAdvancedSettings**](HelmAdvancedSettings.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_environment_helm_status

> models::ReferenceObjectStatusResponseList get_environment_helm_status(environment_id)
List all environment helm statuses

Returns a list of helms with only their id and status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |

### Return type

[**models::ReferenceObjectStatusResponseList**](ReferenceObjectStatusResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_helms

> models::HelmResponseList list_helms(environment_id, to_update)
List helms

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**to_update** | Option<**bool**> | return (or not) results that must be updated |  |[default to false]

### Return type

[**models::HelmResponseList**](HelmResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

