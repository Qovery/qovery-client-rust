# \ApplicationSecretApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_application_secret**](ApplicationSecretApi.md#create_application_secret) | **POST** /application/{applicationId}/secret | Add a secret to the application
[**create_application_secret_alias**](ApplicationSecretApi.md#create_application_secret_alias) | **POST** /application/{applicationId}/secret/{secretId}/alias | Create a secret alias at the application level
[**create_application_secret_override**](ApplicationSecretApi.md#create_application_secret_override) | **POST** /application/{applicationId}/secret/{secretId}/override | Create a secret override at the application level
[**delete_application_secret**](ApplicationSecretApi.md#delete_application_secret) | **DELETE** /application/{applicationId}/secret/{secretId} | Delete a secret from an application
[**edit_application_secret**](ApplicationSecretApi.md#edit_application_secret) | **PUT** /application/{applicationId}/secret/{secretId} | Edit a secret belonging to the application
[**list_application_secrets**](ApplicationSecretApi.md#list_application_secrets) | **GET** /application/{applicationId}/secret | List application secrets



## create_application_secret

> models::Secret create_application_secret(application_id, secret_request)
Add a secret to the application

- Add a secret to the application. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |
**secret_request** | Option<[**SecretRequest**](SecretRequest.md)> |  |  |

### Return type

[**models::Secret**](Secret.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_application_secret_alias

> models::Secret create_application_secret_alias(application_id, secret_id, key)
Create a secret alias at the application level

- Allows you to add an alias at application level on an existing secret having higher scope, in order to customize its key. - You only have to specify a key in the request body - The system will create a new secret at application level with the same value as the one corresponding to the secret id in the path - The response body will contain the newly created secret - Information regarding the aliased_secret will be exposed in the \"aliased_secret\" field of the newly created secret - You can't create an alias on an alias 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |
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


## create_application_secret_override

> models::Secret create_application_secret_override(application_id, secret_id, value)
Create a secret override at the application level

- Allows you to override at application level a secret that has a higher scope. - You only have to specify a value in the request body - The system will create a new secret at application level with the same key as the one corresponding to the secret id in the path - The response body will contain the newly created secret - Information regarding the overridden_secret will be exposed in the \"overridden_secret\" field of the newly created secret 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |
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


## delete_application_secret

> delete_application_secret(application_id, secret_id)
Delete a secret from an application

- To delete a secret you must have the project user permission - You can't delete a BUILT_IN secret - If you delete a secret having override or alias, the associated override/alias will be deleted as well 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |
**secret_id** | **uuid::Uuid** | Secret ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_application_secret

> models::Secret edit_application_secret(application_id, secret_id, secret_edit_request)
Edit a secret belonging to the application

- You can't edit a BUILT_IN secret - For an override, you can't edit the key - For an alias, you can't edit the value - An override can only have a scope lower to the secret it is overriding (hierarchy is BUILT_IN > PROJECT > ENVIRONMENT > APPLICATION) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |
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


## list_application_secrets

> models::SecretResponseList list_application_secrets(application_id)
List application secrets

Secrets are like environment variables, but they are secured and can't be revealed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |

### Return type

[**models::SecretResponseList**](SecretResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

