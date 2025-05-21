# \JobSecretApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_job_secret**](JobSecretApi.md#create_job_secret) | **POST** /job/{jobId}/secret | Add a secret to the job
[**create_job_secret_alias**](JobSecretApi.md#create_job_secret_alias) | **POST** /job/{jobId}/secret/{secretId}/alias | Create a secret alias at the job level
[**create_job_secret_override**](JobSecretApi.md#create_job_secret_override) | **POST** /job/{jobId}/secret/{secretId}/override | Create a secret override at the job level
[**delete_job_secret**](JobSecretApi.md#delete_job_secret) | **DELETE** /job/{jobId}/secret/{secretId} | Delete a secret from an job
[**edit_job_secret**](JobSecretApi.md#edit_job_secret) | **PUT** /job/{jobId}/secret/{secretId} | Edit a secret belonging to the job
[**list_job_secrets**](JobSecretApi.md#list_job_secrets) | **GET** /job/{jobId}/secret | List job secrets



## create_job_secret

> models::Secret create_job_secret(job_id, secret_request)
Add a secret to the job

- Add a secret to the job. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |
**secret_request** | Option<[**SecretRequest**](SecretRequest.md)> |  |  |

### Return type

[**models::Secret**](Secret.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_job_secret_alias

> models::Secret create_job_secret_alias(job_id, secret_id, key)
Create a secret alias at the job level

- Allows you to add an alias at job level on an existing secret having higher scope, in order to customize its key. - You only have to specify a key in the request body - The system will create a new secret at job level with the same value as the one corresponding to the secret id in the path - The response body will contain the newly created secret - Information regarding the aliased_secret will be exposed in the \"aliased_secret\" field of the newly created secret - You can't create an alias on an alias 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |
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


## create_job_secret_override

> models::Secret create_job_secret_override(job_id, secret_id, value)
Create a secret override at the job level

- Allows you to override at job level a secret that has a higher scope. - You only have to specify a value in the request body - The system will create a new secret at job level with the same key as the one corresponding to the secret id in the path - The response body will contain the newly created secret - Information regarding the overridden_secret will be exposed in the \"overridden_secret\" field of the newly created secret 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |
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


## delete_job_secret

> delete_job_secret(job_id, secret_id)
Delete a secret from an job

- To delete a secret you must have the project user permission - You can't delete a BUILT_IN secret - If you delete a secret having override or alias, the associated override/alias will be deleted as well 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |
**secret_id** | **uuid::Uuid** | Secret ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_job_secret

> models::Secret edit_job_secret(job_id, secret_id, secret_edit_request)
Edit a secret belonging to the job

- You can't edit a BUILT_IN secret - For an override, you can't edit the key - For an alias, you can't edit the value - An override can only have a scope lower to the secret it is overriding (hierarchy is BUILT_IN > PROJECT > ENVIRONMENT > CONTAINER) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |
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


## list_job_secrets

> models::SecretResponseList list_job_secrets(job_id)
List job secrets

Secrets are like environment variables, but they are secured and can't be revealed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |

### Return type

[**models::SecretResponseList**](SecretResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

