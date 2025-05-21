# \EnvironmentMainCallsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_environment**](EnvironmentMainCallsApi.md#delete_environment) | **DELETE** /environment/{environmentId} | Delete an environment
[**edit_environment**](EnvironmentMainCallsApi.md#edit_environment) | **PUT** /environment/{environmentId} | Edit an environment
[**get_environment**](EnvironmentMainCallsApi.md#get_environment) | **GET** /environment/{environmentId} | Get environment by ID
[**get_environment_status**](EnvironmentMainCallsApi.md#get_environment_status) | **GET** /environment/{environmentId}/status | Get environment status
[**get_environment_statuses**](EnvironmentMainCallsApi.md#get_environment_statuses) | **GET** /environment/{environmentId}/statuses | Get environment statuses with services status
[**get_environment_statuses_with_stages**](EnvironmentMainCallsApi.md#get_environment_statuses_with_stages) | **GET** /environment/{environmentId}/statusesWithStages | Get environment statuses with stages
[**list_deployment_request_by_environment_id**](EnvironmentMainCallsApi.md#list_deployment_request_by_environment_id) | **GET** /environment/{environmentId}/deploymentQueue | List Deployment Queue Request By EnvironmentId
[**list_deployment_request_by_service_id**](EnvironmentMainCallsApi.md#list_deployment_request_by_service_id) | **GET** /service/{serviceId}/deploymentQueue | List Deployment Queue Request By ServiceId
[**list_services_by_environment_id**](EnvironmentMainCallsApi.md#list_services_by_environment_id) | **GET** /environment/{environmentId}/services | List Services By EnvironmentId



## delete_environment

> delete_environment(environment_id)
Delete an environment

To delete an environment you must have the admin permission

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_environment

> models::Environment edit_environment(environment_id, environment_edit_request)
Edit an environment

To edit an environment you must have the admin permission

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**environment_edit_request** | Option<[**EnvironmentEditRequest**](EnvironmentEditRequest.md)> |  |  |

### Return type

[**models::Environment**](Environment.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_environment

> models::Environment get_environment(environment_id)
Get environment by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |

### Return type

[**models::Environment**](Environment.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_environment_status

> models::EnvironmentStatus get_environment_status(environment_id)
Get environment status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |

### Return type

[**models::EnvironmentStatus**](EnvironmentStatus.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_environment_statuses

> models::EnvironmentStatuses get_environment_statuses(environment_id)
Get environment statuses with services status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |

### Return type

[**models::EnvironmentStatuses**](EnvironmentStatuses.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_environment_statuses_with_stages

> models::EnvironmentStatusesWithStages get_environment_statuses_with_stages(environment_id)
Get environment statuses with stages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |

### Return type

[**models::EnvironmentStatusesWithStages**](EnvironmentStatusesWithStages.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_deployment_request_by_environment_id

> models::ListDeploymentRequestByEnvironmentId200Response list_deployment_request_by_environment_id(environment_id)
List Deployment Queue Request By EnvironmentId

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ListDeploymentRequestByEnvironmentId200Response**](listDeploymentRequestByEnvironmentId_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_deployment_request_by_service_id

> models::ListDeploymentRequestByServiceId200Response list_deployment_request_by_service_id(service_id)
List Deployment Queue Request By ServiceId

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ListDeploymentRequestByServiceId200Response**](listDeploymentRequestByServiceId_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_services_by_environment_id

> models::ListServicesByEnvironmentId200Response list_services_by_environment_id(environment_id)
List Services By EnvironmentId

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **String** |  | [required] |

### Return type

[**models::ListServicesByEnvironmentId200Response**](listServicesByEnvironmentId_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

