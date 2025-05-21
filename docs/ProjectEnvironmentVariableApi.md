# \ProjectEnvironmentVariableApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_project_environment_variable**](ProjectEnvironmentVariableApi.md#create_project_environment_variable) | **POST** /project/{projectId}/environmentVariable | Add an environment variable to the project
[**create_project_environment_variable_alias**](ProjectEnvironmentVariableApi.md#create_project_environment_variable_alias) | **POST** /project/{projectId}/environmentVariable/{environmentVariableId}/alias | Create an environment variable alias at the project level
[**create_project_environment_variable_override**](ProjectEnvironmentVariableApi.md#create_project_environment_variable_override) | **POST** /project/{projectId}/environmentVariable/{environmentVariableId}/override | Create an environment variable override at the project level
[**delete_project_environment_variable**](ProjectEnvironmentVariableApi.md#delete_project_environment_variable) | **DELETE** /project/{projectId}/environmentVariable/{environmentVariableId} | Delete an environment variable from a project
[**edit_project_environment_variable**](ProjectEnvironmentVariableApi.md#edit_project_environment_variable) | **PUT** /project/{projectId}/environmentVariable/{environmentVariableId} | Edit an environment variable belonging to the project
[**list_project_environment_variable**](ProjectEnvironmentVariableApi.md#list_project_environment_variable) | **GET** /project/{projectId}/environmentVariable | List project environment variables



## create_project_environment_variable

> models::EnvironmentVariable create_project_environment_variable(project_id, environment_variable_request)
Add an environment variable to the project

- Add an environment variable to the project.   - If the environment variable key already exists, then it will be replaced by the new one.   - If the environment variable value points toward an existing environment variable key, it will be considered as an alias. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |
**environment_variable_request** | Option<[**EnvironmentVariableRequest**](EnvironmentVariableRequest.md)> |  |  |

### Return type

[**models::EnvironmentVariable**](EnvironmentVariable.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_project_environment_variable_alias

> models::EnvironmentVariable create_project_environment_variable_alias(project_id, environment_variable_id, key)
Create an environment variable alias at the project level

- Allows you to add an alias at project level on an existing environment variable having higher scope, in order to customize its key. - You only have to specify a key in the request body - The system will create a new environment variable at project level with the same value as the one corresponding to the variable id in the path - The response body will contain the newly created variable - Information regarding the aliased_variable will be exposed in the \"aliased_variable\" field of the newly created variable - You can't create an alias on an alias 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |
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


## create_project_environment_variable_override

> models::EnvironmentVariable create_project_environment_variable_override(project_id, environment_variable_id, value)
Create an environment variable override at the project level

- Allows you to override at project level an environment variable that has a higher scope. - You only have to specify a value in the request body - The system will create a new environment variable at project level with the same key as the one corresponding to the variable id in the path - The response body will contain the newly created variable - Information regarding the overridden_variable will be exposed in the \"overridden_variable\" field of the newly created variable 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |
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


## delete_project_environment_variable

> delete_project_environment_variable(project_id, environment_variable_id)
Delete an environment variable from a project

- To delete an environment variable you must have the project user permission - You can't delete a BUILT_IN variable - If you delete a variable having override or alias, the associated override/alias will be deleted as well 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |
**environment_variable_id** | **uuid::Uuid** | Environment Variable ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_project_environment_variable

> models::EnvironmentVariable edit_project_environment_variable(project_id, environment_variable_id, environment_variable_edit_request)
Edit an environment variable belonging to the project

- You can't edit a BUILT_IN variable - For an override, you can't edit the key - For an alias, you can't edit the value - An override can only have a scope lower to the variable it is overriding (hierarchy is BUILT_IN > PROJECT > ENVIRONMENT > APPLICATION) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |
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


## list_project_environment_variable

> models::EnvironmentVariableResponseList list_project_environment_variable(project_id)
List project environment variables

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |

### Return type

[**models::EnvironmentVariableResponseList**](EnvironmentVariableResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

