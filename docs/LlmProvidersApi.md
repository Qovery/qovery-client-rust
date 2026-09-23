# \LlmProvidersApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_llm_provider**](LlmProvidersApi.md#create_llm_provider) | **POST** /organization/{organizationId}/llmProvider | Create an LLM provider
[**delete_llm_provider**](LlmProvidersApi.md#delete_llm_provider) | **DELETE** /llmProvider/{llmProviderId} | Delete an LLM provider
[**edit_llm_provider**](LlmProvidersApi.md#edit_llm_provider) | **PUT** /llmProvider/{llmProviderId} | Edit an LLM provider
[**get_llm_provider**](LlmProvidersApi.md#get_llm_provider) | **GET** /llmProvider/{llmProviderId} | Get an LLM provider
[**list_llm_provider_models**](LlmProvidersApi.md#list_llm_provider_models) | **GET** /llmProvider/{llmProviderId}/models | List the models of an LLM provider
[**list_llm_providers**](LlmProvidersApi.md#list_llm_providers) | **GET** /organization/{organizationId}/llmProvider | List organization LLM providers



## create_llm_provider

> models::LlmProviderResponse create_llm_provider(organization_id, llm_provider_request)
Create an LLM provider

Configure a reusable LLM provider for an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**llm_provider_request** | [**LlmProviderRequest**](LlmProviderRequest.md) |  | [required] |

### Return type

[**models::LlmProviderResponse**](LlmProviderResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_llm_provider

> delete_llm_provider(llm_provider_id)
Delete an LLM provider

Delete an LLM provider.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**llm_provider_id** | **uuid::Uuid** | LLM Provider ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_llm_provider

> models::LlmProviderResponse edit_llm_provider(llm_provider_id, llm_provider_request)
Edit an LLM provider

Replace an LLM provider. Sending a blank credential keeps the stored one; sending a nonblank credential rotates it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**llm_provider_id** | **uuid::Uuid** | LLM Provider ID | [required] |
**llm_provider_request** | [**LlmProviderRequest**](LlmProviderRequest.md) |  | [required] |

### Return type

[**models::LlmProviderResponse**](LlmProviderResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_llm_provider

> models::LlmProviderResponse get_llm_provider(llm_provider_id)
Get an LLM provider

Get an LLM provider. The credential is never returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**llm_provider_id** | **uuid::Uuid** | LLM Provider ID | [required] |

### Return type

[**models::LlmProviderResponse**](LlmProviderResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_llm_provider_models

> models::LlmProviderModelResponseList list_llm_provider_models(llm_provider_id)
List the models of an LLM provider

List the models the provider's stored credential can reach, fetched live from the provider. CLAUDE lists Anthropic models; BEDROCK lists Anthropic inference profiles in us-east-1. The credential is never returned. A USER provider can only be listed by its owner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**llm_provider_id** | **uuid::Uuid** | LLM Provider ID | [required] |

### Return type

[**models::LlmProviderModelResponseList**](LlmProviderModelResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_llm_providers

> models::LlmProviderResponseList list_llm_providers(organization_id)
List organization LLM providers

List the LLM providers configured for an organization. Credentials are never returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::LlmProviderResponseList**](LlmProviderResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

