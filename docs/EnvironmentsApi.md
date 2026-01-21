# \EnvironmentsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_environment**](EnvironmentsApi.md#create_environment) | **POST** /project/{projectId}/environment | Create an environment
[**get_project_environment_service_number**](EnvironmentsApi.md#get_project_environment_service_number) | **GET** /project/{projectId}/environment/stats | List total number of services for each environment of the project
[**get_project_environments_overview**](EnvironmentsApi.md#get_project_environments_overview) | **GET** /project/{projectId}/environmentOverview | List environments overview
[**get_project_environments_status**](EnvironmentsApi.md#get_project_environments_status) | **GET** /project/{projectId}/environment/status | List environments statuses
[**list_environment**](EnvironmentsApi.md#list_environment) | **GET** /project/{projectId}/environment | List environments



## create_environment

> models::Environment create_environment(project_id, create_environment_request)
Create an environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |
**create_environment_request** | Option<[**CreateEnvironmentRequest**](CreateEnvironmentRequest.md)> |  |  |

### Return type

[**models::Environment**](Environment.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_environment_service_number

> models::EnvironmentStatsResponseList get_project_environment_service_number(project_id)
List total number of services for each environment of the project

Returns a list of environment ids, and for each its total numberof services

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |

### Return type

[**models::EnvironmentStatsResponseList**](EnvironmentStatsResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_environments_overview

> models::EnvironmentOverviewResponseList get_project_environments_overview(project_id)
List environments overview

Returns a list of environments with their overview information including deployment status, service count, and cluster details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |

### Return type

[**models::EnvironmentOverviewResponseList**](EnvironmentOverviewResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_environments_status

> models::EnvironmentStatusList get_project_environments_status(project_id)
List environments statuses

Returns a list of environments with only their id and status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |

### Return type

[**models::EnvironmentStatusList**](EnvironmentStatusList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_environment

> models::EnvironmentResponseList list_environment(project_id)
List environments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |

### Return type

[**models::EnvironmentResponseList**](EnvironmentResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

