# \ContainerEnvironmentVariableApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_container_environment_variable**](ContainerEnvironmentVariableApi.md#create_container_environment_variable) | **POST** /container/{containerId}/environmentVariable | Add an environment variable to the container
[**create_container_environment_variable_alias**](ContainerEnvironmentVariableApi.md#create_container_environment_variable_alias) | **POST** /container/{containerId}/environmentVariable/{environmentVariableId}/alias | Create an environment variable alias at the container level
[**create_container_environment_variable_override**](ContainerEnvironmentVariableApi.md#create_container_environment_variable_override) | **POST** /container/{containerId}/environmentVariable/{environmentVariableId}/override | Create an environment variable override at the container level
[**delete_container_environment_variable**](ContainerEnvironmentVariableApi.md#delete_container_environment_variable) | **DELETE** /container/{containerId}/environmentVariable/{environmentVariableId} | Delete an environment variable from a container
[**edit_container_environment_variable**](ContainerEnvironmentVariableApi.md#edit_container_environment_variable) | **PUT** /container/{containerId}/environmentVariable/{environmentVariableId} | Edit an environment variable belonging to the container
[**import_container_environment_variable**](ContainerEnvironmentVariableApi.md#import_container_environment_variable) | **POST** /container/{containerId}/environmentVariable/import | Import variables
[**list_container_environment_variable**](ContainerEnvironmentVariableApi.md#list_container_environment_variable) | **GET** /container/{containerId}/environmentVariable | List environment variables



## create_container_environment_variable

> models::EnvironmentVariable create_container_environment_variable(container_id, environment_variable_request)
Add an environment variable to the container

- Add an environment variable to the container. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** | Container ID | [required] |
**environment_variable_request** | Option<[**EnvironmentVariableRequest**](EnvironmentVariableRequest.md)> |  |  |

### Return type

[**models::EnvironmentVariable**](EnvironmentVariable.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_container_environment_variable_alias

> models::EnvironmentVariable create_container_environment_variable_alias(container_id, environment_variable_id, key)
Create an environment variable alias at the container level

- Allows you to add an alias at container level on an existing environment variable having higher scope, in order to customize its key. - You only have to specify a key in the request body - The system will create a new environment variable at container level with the same value as the one corresponding to the variable id in the path - The response body will contain the newly created variable - Information regarding the aliased_variable will be exposed in the \"aliased_variable\" field of the newly created variable - You can't create an alias on an alias 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** | Container ID | [required] |
**environment_variable_id** | **uuid::Uuid** | Environment Variable ID | [required] |
**key** | Option<[**Key**](Key.md)> |  |  |

### Return type

[**models::EnvironmentVariable**](EnvironmentVariable.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_container_environment_variable_override

> models::EnvironmentVariable create_container_environment_variable_override(container_id, environment_variable_id, value)
Create an environment variable override at the container level

- Allows you to override at container level an environment variable that has a higher scope. - You only have to specify a value in the request body - The system will create a new environment variable at container level with the same key as the one corresponding to the variable id in the path - The response body will contain the newly created variable - Information regarding the overridden_variable will be exposed in the \"overridden_variable\" field of the newly created variable 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** | Container ID | [required] |
**environment_variable_id** | **uuid::Uuid** | Environment Variable ID | [required] |
**value** | Option<[**Value**](Value.md)> |  |  |

### Return type

[**models::EnvironmentVariable**](EnvironmentVariable.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_container_environment_variable

> delete_container_environment_variable(container_id, environment_variable_id)
Delete an environment variable from a container

- To delete an environment variable from an container you must have the project user permission - You can't delete a BUILT_IN variable - If you delete a variable having override or alias, the associated override/alias will be deleted as well 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** | Container ID | [required] |
**environment_variable_id** | **uuid::Uuid** | Environment Variable ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_container_environment_variable

> models::EnvironmentVariable edit_container_environment_variable(container_id, environment_variable_id, environment_variable_edit_request)
Edit an environment variable belonging to the container

- You can't edit a BUILT_IN variable - For an override, you can't edit the key - For an alias, you can't edit the value - An override can only have a scope lower to the variable it is overriding (hierarchy is BUILT_IN > PROJECT > ENVIRONMENT > CONTAINER) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** | Container ID | [required] |
**environment_variable_id** | **uuid::Uuid** | Environment Variable ID | [required] |
**environment_variable_edit_request** | [**EnvironmentVariableEditRequest**](EnvironmentVariableEditRequest.md) |  | [required] |

### Return type

[**models::EnvironmentVariable**](EnvironmentVariable.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_container_environment_variable

> models::VariableImport import_container_environment_variable(container_id, variable_import_request)
Import variables

Import environment variables in a defined scope, with a defined visibility.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** | Container ID | [required] |
**variable_import_request** | Option<[**VariableImportRequest**](VariableImportRequest.md)> |  |  |

### Return type

[**models::VariableImport**](VariableImport.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_container_environment_variable

> models::EnvironmentVariableResponseList list_container_environment_variable(container_id)
List environment variables

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** | Container ID | [required] |

### Return type

[**models::EnvironmentVariableResponseList**](EnvironmentVariableResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

