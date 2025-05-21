# \ProjectSecretApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_project_secret**](ProjectSecretApi.md#create_project_secret) | **POST** /project/{projectId}/secret | Add a secret to the project
[**create_project_secret_alias**](ProjectSecretApi.md#create_project_secret_alias) | **POST** /project/{projectId}/secret/{secretId}/alias | Create a secret alias at the project level
[**create_project_secret_override**](ProjectSecretApi.md#create_project_secret_override) | **POST** /project/{projectId}/secret/{secretId}/override | Create a secret override at the project level
[**delete_project_secret**](ProjectSecretApi.md#delete_project_secret) | **DELETE** /project/{projectId}/secret/{secretId} | Delete a secret from a project
[**edit_project_secret**](ProjectSecretApi.md#edit_project_secret) | **PUT** /project/{projectId}/secret/{secretId} | Edit a secret belonging to the project
[**list_project_secrets**](ProjectSecretApi.md#list_project_secrets) | **GET** /project/{projectId}/secret | List project secrets



## create_project_secret

> models::Secret create_project_secret(project_id, secret_request)
Add a secret to the project

- Add a secret to the project.   - If the secret key already exists, then it will be replaced by the new one.   - If the secret value points toward an existing secret key, it will be considered as an alias. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |
**secret_request** | Option<[**SecretRequest**](SecretRequest.md)> |  |  |

### Return type

[**models::Secret**](Secret.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_project_secret_alias

> models::Secret create_project_secret_alias(project_id, secret_id, key)
Create a secret alias at the project level

- Allows you to add an alias at project level on an existing secret having higher scope, in order to customize its key. - You only have to specify a key in the request body - The system will create a new secret at project level with the same value as the one corresponding to the secret id in the path - The response body will contain the newly created secret - Information regarding the aliased_secret will be exposed in the \"aliased_secret\" field of the newly created secret - You can't create an alias on an alias 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |
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


## create_project_secret_override

> models::Secret create_project_secret_override(project_id, secret_id, value)
Create a secret override at the project level

- Allows you to override at project level a secret that has a higher scope. - You only have to specify a value in the request body - The system will create a new secret at project level with the same key as the one corresponding to the secret id in the path - The response body will contain the newly created secret - Information regarding the overridden_secret will be exposed in the \"overridden_secret\" field of the newly created secret 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |
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


## delete_project_secret

> delete_project_secret(project_id, secret_id)
Delete a secret from a project

- To delete a secret you must have the project user permission - You can't delete a BUILT_IN secret - If you delete a secret having override or alias, the associated override/alias will be deleted as well  operationId: deleteProjectSecret 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |
**secret_id** | **uuid::Uuid** | Secret ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_project_secret

> models::Secret edit_project_secret(project_id, secret_id, secret_edit_request)
Edit a secret belonging to the project

- You can't edit a BUILT_IN secret - For an override, you can't edit the key - For an alias, you can't edit the value - An override can only have a scope lower to the secret it is overriding (hierarchy is BUILT_IN > PROJECT > ENVIRONMENT > APPLICATION) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |
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


## list_project_secrets

> models::SecretResponseList list_project_secrets(project_id)
List project secrets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |

### Return type

[**models::SecretResponseList**](SecretResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

