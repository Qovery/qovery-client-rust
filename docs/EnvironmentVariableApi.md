# \EnvironmentVariableApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_environment_environment_variable**](EnvironmentVariableApi.md#create_environment_environment_variable) | **POST** /environment/{environmentId}/environmentVariable | Add an environment variable to the environment
[**create_environment_environment_variable_alias**](EnvironmentVariableApi.md#create_environment_environment_variable_alias) | **POST** /environment/{environmentId}/environmentVariable/{environmentVariableId}/alias | Create an environment variable alias at the environment level
[**create_environment_environment_variable_override**](EnvironmentVariableApi.md#create_environment_environment_variable_override) | **POST** /environment/{environmentId}/environmentVariable/{environmentVariableId}/override | Create an environment variable override at the environment level
[**delete_environment_environment_variable**](EnvironmentVariableApi.md#delete_environment_environment_variable) | **DELETE** /environment/{environmentId}/environmentVariable/{environmentVariableId} | Delete an environment variable from an environment
[**edit_environment_environment_variable**](EnvironmentVariableApi.md#edit_environment_environment_variable) | **PUT** /environment/{environmentId}/environmentVariable/{environmentVariableId} | Edit an environment variable belonging to the environment
[**list_environment_environment_variable**](EnvironmentVariableApi.md#list_environment_environment_variable) | **GET** /environment/{environmentId}/environmentVariable | List environment variables



## create_environment_environment_variable

> models::EnvironmentVariable create_environment_environment_variable(environment_id, environment_variable_request)
Add an environment variable to the environment

- Add an environment variable to the environment.   - If the environment variable key already exists, then it will be replaced by the new one.   - If the environment variable value points toward an existing environment variable key, it will be considered as an alias. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**environment_variable_request** | Option<[**EnvironmentVariableRequest**](EnvironmentVariableRequest.md)> |  |  |

### Return type

[**models::EnvironmentVariable**](EnvironmentVariable.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_environment_environment_variable_alias

> models::EnvironmentVariable create_environment_environment_variable_alias(environment_id, environment_variable_id, key)
Create an environment variable alias at the environment level

- Allows you to add an alias at environment level on an existing environment variable having higher scope, in order to customize its key. - You only have to specify a key in the request body - The system will create a new environment variable at environment level with the same value as the one corresponding to the variable id in the path - The response body will contain the newly created variable - Information regarding the aliased_variable will be exposed in the \"aliased_variable\" field of the newly created variable - You can't create an alias on an alias 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
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


## create_environment_environment_variable_override

> models::EnvironmentVariable create_environment_environment_variable_override(environment_id, environment_variable_id, value)
Create an environment variable override at the environment level

- Allows you to override at environment level an environment variable that has a higher scope. - You only have to specify a value in the request body - The system will create a new environment variable at environment level with the same key as the one corresponding to the variable id in the path - The response body will contain the newly created variable - Information regarding the overridden_variable will be exposed in the \"overridden_variable\" field of the newly created variable 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
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


## delete_environment_environment_variable

> delete_environment_environment_variable(environment_id, environment_variable_id)
Delete an environment variable from an environment

- To delete an environment variable you must have the project user permission - You can't delete a BUILT_IN variable - If you delete a variable having override or alias, the associated override/alias will be deleted as well 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**environment_variable_id** | **uuid::Uuid** | Environment Variable ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_environment_environment_variable

> models::EnvironmentVariable edit_environment_environment_variable(environment_id, environment_variable_id, environment_variable_edit_request)
Edit an environment variable belonging to the environment

- You can't edit a BUILT_IN variable - For an override, you can't edit the key - For an alias, you can't edit the value - An override can only have a scope lower to the variable it is overriding (hierarchy is BUILT_IN > PROJECT > ENVIRONMENT > APPLICATION) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
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


## list_environment_environment_variable

> models::EnvironmentVariableResponseList list_environment_environment_variable(environment_id)
List environment variables

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |

### Return type

[**models::EnvironmentVariableResponseList**](EnvironmentVariableResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

