# \VariableMainCallsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_variable**](VariableMainCallsApi.md#create_variable) | **POST** /variable | Create a variable
[**create_variable_alias**](VariableMainCallsApi.md#create_variable_alias) | **POST** /variable/{variableId}/alias | Create a variable alias
[**create_variable_override**](VariableMainCallsApi.md#create_variable_override) | **POST** /variable/{variableId}/override | Create a variable override
[**delete_variable**](VariableMainCallsApi.md#delete_variable) | **DELETE** /variable/{variableId} | Delete a variable
[**edit_variable**](VariableMainCallsApi.md#edit_variable) | **PUT** /variable/{variableId} | Edit a variable
[**import_environment_variables**](VariableMainCallsApi.md#import_environment_variables) | **POST** /variable/import | Import variables
[**list_variables**](VariableMainCallsApi.md#list_variables) | **GET** /variable | List variables



## create_variable

> models::VariableResponse create_variable(variable_request)
Create a variable

- Create a variable with the scope defined in the request body. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**variable_request** | Option<[**VariableRequest**](VariableRequest.md)> |  |  |

### Return type

[**models::VariableResponse**](VariableResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_variable_alias

> models::VariableResponse create_variable_alias(variable_id, variable_alias_request)
Create a variable alias

- Allows you to create an alias of one of the existing variables. - You have to specify an alias (key) in the request body, the scope and the parent id of the alias (project id, environment id or service id) - The system will create a new variable at the requested level with the same value as the one corresponding to the variable id passed as path parameter. - The response body will contain the newly created variable - Information regarding the aliased_variable will be exposed in the \"aliased_variable\" or in the \"aliased_secret\" field of the newly created variable - You can't create an alias on an alias 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**variable_id** | **uuid::Uuid** | Variable ID | [required] |
**variable_alias_request** | Option<[**VariableAliasRequest**](VariableAliasRequest.md)> |  |  |

### Return type

[**models::VariableResponse**](VariableResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_variable_override

> models::VariableResponse create_variable_override(variable_id, variable_override_request)
Create a variable override

- Allows you to override a variable that has a higher scope. - You have to specify a value (override) in the request body and the scope and the parent id of the variable to override (project id, environment id or service id) - The system will create a new environment variable at the requested level with the same key as the one corresponding to the variable id passed as path parameter. - The response body will contain the newly created variable - Information regarding the overridden_variable will be exposed in the \"overridden_variable\" or in the \"overridden_secret\" field of the newly created variable 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**variable_id** | **uuid::Uuid** | Variable ID | [required] |
**variable_override_request** | Option<[**VariableOverrideRequest**](VariableOverrideRequest.md)> |  |  |

### Return type

[**models::VariableResponse**](VariableResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_variable

> delete_variable(variable_id)
Delete a variable

- To delete a variable - You can't delete a BUILT_IN variable - If you delete a variable having override or alias, the associated override/alias will be deleted as well 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**variable_id** | **uuid::Uuid** | Variable ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_variable

> models::VariableResponse edit_variable(variable_id, variable_edit_request)
Edit a variable

- You can't edit a BUILT_IN variable - For an override, you can't edit the key - For an alias, you can't edit the value 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**variable_id** | **uuid::Uuid** | Variable ID | [required] |
**variable_edit_request** | [**VariableEditRequest**](VariableEditRequest.md) |  | [required] |

### Return type

[**models::VariableResponse**](VariableResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_environment_variables

> models::VariableImport import_environment_variables(service_id, service_type, variable_import_request)
Import variables

Import environment variables in a defined scope, with a defined visibility.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **uuid::Uuid** | service id | [required] |
**service_type** | [**ServiceTypeForVariableEnum**](.md) | service type | [required] |
**variable_import_request** | Option<[**VariableImportRequest**](VariableImportRequest.md)> |  |  |

### Return type

[**models::VariableImport**](VariableImport.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_variables

> models::VariableResponseList list_variables(parent_id, scope, is_secret)
List variables

Returns a list of variables. The result can be filtered by using the query parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_id** | **uuid::Uuid** | it filters the list by returning only the variables accessible by the selected parent_id. This field shall contain the id of a project, environment or service depending on the selected scope. Example, if scope = APPLICATION and parent_id=<application_id>, the result will contain any variable accessible by the application. The result will contain also any variable declared at an higher scope. | [required] |
**scope** | [**ApiVariableScopeEnum**](.md) | the type of the parent_id (application, project, environment etc..). | [required] |
**is_secret** | Option<**bool**> |  |  |

### Return type

[**models::VariableResponseList**](VariableResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

