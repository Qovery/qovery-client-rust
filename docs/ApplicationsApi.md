# \ApplicationsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clone_application**](ApplicationsApi.md#clone_application) | **POST** /application/{applicationId}/clone | Clone application
[**create_application**](ApplicationsApi.md#create_application) | **POST** /environment/{environmentId}/application | Create an application
[**get_default_application_advanced_settings**](ApplicationsApi.md#get_default_application_advanced_settings) | **GET** /defaultApplicationAdvancedSettings | List default application advanced settings
[**get_environment_application_status**](ApplicationsApi.md#get_environment_application_status) | **GET** /environment/{environmentId}/application/status | List all environment applications statuses
[**list_application**](ApplicationsApi.md#list_application) | **GET** /environment/{environmentId}/application | List applications



## clone_application

> models::Application clone_application(application_id, clone_service_request)
Clone application

This will create a new application with the same configuration on the targeted environment Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |
**clone_service_request** | Option<[**CloneServiceRequest**](CloneServiceRequest.md)> |  |  |

### Return type

[**models::Application**](Application.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_application

> models::Application create_application(environment_id, application_request)
Create an application

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**application_request** | Option<[**ApplicationRequest**](ApplicationRequest.md)> |  |  |

### Return type

[**models::Application**](Application.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_application_advanced_settings

> models::ApplicationAdvancedSettings get_default_application_advanced_settings()
List default application advanced settings

Default values for each setting are available in [our documentation](https://hub.qovery.com/docs/using-qovery/configuration/advanced-settings/)

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApplicationAdvancedSettings**](ApplicationAdvancedSettings.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_environment_application_status

> models::ReferenceObjectStatusResponseList get_environment_application_status(environment_id)
List all environment applications statuses

Returns a list of applications with only their id and status.

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


## list_application

> models::ApplicationResponseList list_application(environment_id, to_update)
List applications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**to_update** | Option<**bool**> | return (or not) results that must be updated |  |[default to false]

### Return type

[**models::ApplicationResponseList**](ApplicationResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

