# \SecretManagerAccessApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_secret_manager_access_external_secrets**](SecretManagerAccessApi.md#list_secret_manager_access_external_secrets) | **GET** /api/secretManagerAccess/{secretManagerAccessId}/associatedServices | List external secrets used by a secret manager access
[**list_upstream_secrets_from_secret_provider**](SecretManagerAccessApi.md#list_upstream_secrets_from_secret_provider) | **GET** /api/secretManagerAccess/{secretManagerAccessId}/secrets | List upstream secrets from secret provider



## list_secret_manager_access_external_secrets

> models::ExternalSecretAssociatedServiceResponseList list_secret_manager_access_external_secrets(secret_manager_access_id)
List external secrets used by a secret manager access

List external secrets used by a secret manager access

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_manager_access_id** | **uuid::Uuid** | Secret Manager Access ID | [required] |

### Return type

[**models::ExternalSecretAssociatedServiceResponseList**](ExternalSecretAssociatedServiceResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_upstream_secrets_from_secret_provider

> models::ProviderSecrets list_upstream_secrets_from_secret_provider(secret_manager_access_id, name_prefix)
List upstream secrets from secret provider

List upstream secrets from secret provider

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_manager_access_id** | **uuid::Uuid** | Secret Manager Access ID | [required] |
**name_prefix** | Option<**String**> |  |  |

### Return type

[**models::ProviderSecrets**](ProviderSecrets.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

