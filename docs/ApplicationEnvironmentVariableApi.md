# \ApplicationEnvironmentVariableApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_application_environment_variable**](ApplicationEnvironmentVariableApi.md#create_application_environment_variable) | **POST** /application/{applicationId}/environmentVariable | Add an environment variable to the application
[**create_application_environment_variable_alias**](ApplicationEnvironmentVariableApi.md#create_application_environment_variable_alias) | **POST** /application/{applicationId}/environmentVariable/{environmentVariableId}/alias | Create an environment variable alias at the application level
[**create_application_environment_variable_override**](ApplicationEnvironmentVariableApi.md#create_application_environment_variable_override) | **POST** /application/{applicationId}/environmentVariable/{environmentVariableId}/override | Create an environment variable override at the application level
[**delete_application_environment_variable**](ApplicationEnvironmentVariableApi.md#delete_application_environment_variable) | **DELETE** /application/{applicationId}/environmentVariable/{environmentVariableId} | Delete an environment variable from an application
[**edit_application_environment_variable**](ApplicationEnvironmentVariableApi.md#edit_application_environment_variable) | **PUT** /application/{applicationId}/environmentVariable/{environmentVariableId} | Edit an environment variable belonging to the application
[**import_environment_variable**](ApplicationEnvironmentVariableApi.md#import_environment_variable) | **POST** /application/{applicationId}/environmentVariable/import | Import variables
[**list_application_environment_variable**](ApplicationEnvironmentVariableApi.md#list_application_environment_variable) | **GET** /application/{applicationId}/environmentVariable | List environment variables



## create_application_environment_variable

> models::EnvironmentVariable create_application_environment_variable(application_id, environment_variable_request)
Add an environment variable to the application

- Add an environment variable to the application. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |
**environment_variable_request** | Option<[**EnvironmentVariableRequest**](EnvironmentVariableRequest.md)> |  |  |

### Return type

[**models::EnvironmentVariable**](EnvironmentVariable.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_application_environment_variable_alias

> models::EnvironmentVariable create_application_environment_variable_alias(application_id, environment_variable_id, key)
Create an environment variable alias at the application level

- Allows you to add an alias at application level on an existing environment variable having higher scope, in order to customize its key. - You only have to specify a key in the request body - The system will create a new environment variable at application level with the same value as the one corresponding to the variable id in the path - The response body will contain the newly created variable - Information regarding the aliased_variable will be exposed in the \"aliased_variable\" field of the newly created variable - You can't create an alias on an alias 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |
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


## create_application_environment_variable_override

> models::EnvironmentVariable create_application_environment_variable_override(application_id, environment_variable_id, value)
Create an environment variable override at the application level

- Allows you to override at application level an environment variable that has a higher scope. - You only have to specify a value in the request body - The system will create a new environment variable at application level with the same key as the one corresponding to the variable id in the path - The response body will contain the newly created variable - Information regarding the overridden_variable will be exposed in the \"overridden_variable\" field of the newly created variable 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |
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


## delete_application_environment_variable

> delete_application_environment_variable(application_id, environment_variable_id)
Delete an environment variable from an application

- To delete an environment variable from an application you must have the project user permission - You can't delete a BUILT_IN variable - If you delete a variable having override or alias, the associated override/alias will be deleted as well 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |
**environment_variable_id** | **uuid::Uuid** | Environment Variable ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_application_environment_variable

> models::EnvironmentVariable edit_application_environment_variable(application_id, environment_variable_id, environment_variable_edit_request)
Edit an environment variable belonging to the application

- You can't edit a BUILT_IN variable - For an override, you can't edit the key - For an alias, you can't edit the value - An override can only have a scope lower to the variable it is overriding (hierarchy is BUILT_IN > PROJECT > ENVIRONMENT > APPLICATION) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |
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


## import_environment_variable

> models::VariableImport import_environment_variable(application_id, variable_import_request)
Import variables

Import environment variables in a defined scope, with a defined visibility.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |
**variable_import_request** | Option<[**VariableImportRequest**](VariableImportRequest.md)> |  |  |

### Return type

[**models::VariableImport**](VariableImport.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_application_environment_variable

> models::EnvironmentVariableResponseList list_application_environment_variable(application_id)
List environment variables

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |

### Return type

[**models::EnvironmentVariableResponseList**](EnvironmentVariableResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

