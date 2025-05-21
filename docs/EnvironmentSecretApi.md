# \EnvironmentSecretApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_environment_secret**](EnvironmentSecretApi.md#create_environment_secret) | **POST** /environment/{environmentId}/secret | Add a secret to the environment
[**create_environment_secret_alias**](EnvironmentSecretApi.md#create_environment_secret_alias) | **POST** /environment/{environmentId}/secret/{secretId}/alias | Create a secret alias at the environment level
[**create_environment_secret_override**](EnvironmentSecretApi.md#create_environment_secret_override) | **POST** /environment/{environmentId}/secret/{secretId}/override | Create a secret override at the environment level
[**delete_environment_secret**](EnvironmentSecretApi.md#delete_environment_secret) | **DELETE** /environment/{environmentId}/secret/{secretId} | Delete a secret from the environment
[**edit_environment_secret**](EnvironmentSecretApi.md#edit_environment_secret) | **PUT** /environment/{environmentId}/secret/{secretId} | Edit a secret belonging to the environment
[**list_environment_secrets**](EnvironmentSecretApi.md#list_environment_secrets) | **GET** /environment/{environmentId}/secret | List environment secrets



## create_environment_secret

> models::Secret create_environment_secret(environment_id, secret_request)
Add a secret to the environment

- Add a secret to the environment.   - If the secret key already exists, then it will be replaced by the new one.   - If the secret value points toward an existing secret key, it will be considered as an alias. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**secret_request** | Option<[**SecretRequest**](SecretRequest.md)> |  |  |

### Return type

[**models::Secret**](Secret.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_environment_secret_alias

> models::Secret create_environment_secret_alias(environment_id, secret_id, key)
Create a secret alias at the environment level

- Allows you to add an alias at environment level on an existing secret having higher scope, in order to customize its key. - You only have to specify a key in the request body - The system will create a new secret at environment level with the same value as the one corresponding to the secret id in the path - The response body will contain the newly created secret - Information regarding the aliased_secret will be exposed in the \"aliased_secret\" field of the newly created secret - You can't create an alias on an alias 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**secret_id** | **uuid::Uuid** | Secret ID | [required] |
**key** | Option<[**Key**](Key.md)> |  |  |

### Return type

[**models::Secret**](Secret.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_environment_secret_override

> models::Secret create_environment_secret_override(environment_id, secret_id, value)
Create a secret override at the environment level

- Allows you to override at environment level a secret that has a higher scope. - You only have to specify a value in the request body - The system will create a new secret at environment level with the same key as the one corresponding to the secret id in the path - The response body will contain the newly created secret - Information regarding the overridden_secret will be exposed in the \"overridden_secret\" field of the newly created secret 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**secret_id** | **uuid::Uuid** | Secret ID | [required] |
**value** | Option<[**Value**](Value.md)> |  |  |

### Return type

[**models::Secret**](Secret.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_environment_secret

> delete_environment_secret(environment_id, secret_id)
Delete a secret from the environment

- To delete a secret you must have the project user permission - You can't delete a BUILT_IN secret - If you delete a secret having override or alias, the associated override/alias will be deleted as well  operationId: deleteEnvironmentSecret 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**secret_id** | **uuid::Uuid** | Secret ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_environment_secret

> models::Secret edit_environment_secret(environment_id, secret_id, secret_edit_request)
Edit a secret belonging to the environment

- You can't edit a BUILT_IN secret - For an override, you can't edit the key - For an alias, you can't edit the value - An override can only have a scope lower to the secret it is overriding (hierarchy is BUILT_IN > PROJECT > ENVIRONMENT > APPLICATION) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**secret_id** | **uuid::Uuid** | Secret ID | [required] |
**secret_edit_request** | [**SecretEditRequest**](SecretEditRequest.md) |  | [required] |

### Return type

[**models::Secret**](Secret.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_environment_secrets

> models::SecretResponseList list_environment_secrets(environment_id)
List environment secrets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |

### Return type

[**models::SecretResponseList**](SecretResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

