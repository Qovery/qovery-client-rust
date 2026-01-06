# \KedaTriggerAuthenticationApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_keda_trigger_authentication**](KedaTriggerAuthenticationApi.md#create_keda_trigger_authentication) | **POST** /organization/{organizationId}/keda-trigger-authentications | create Keda Trigger Authentications
[**delete_keda_trigger_authentication**](KedaTriggerAuthenticationApi.md#delete_keda_trigger_authentication) | **DELETE** /organization/{organizationId}/keda-trigger-authentications/{triggerAuthenticationId} | Delete a KEDA trigger authentication
[**get_keda_trigger_authentication**](KedaTriggerAuthenticationApi.md#get_keda_trigger_authentication) | **GET** /organization/{organizationId}/keda-trigger-authentications/{triggerAuthenticationId} | Get a KEDA trigger authentication
[**list_keda_trigger_authentications**](KedaTriggerAuthenticationApi.md#list_keda_trigger_authentications) | **GET** /organization/{organizationId}/keda-trigger-authentications | list Keda TriggerAuthentications
[**update_keda_trigger_authentication**](KedaTriggerAuthenticationApi.md#update_keda_trigger_authentication) | **PUT** /organization/{organizationId}/keda-trigger-authentications/{triggerAuthenticationId} | Update a KEDA trigger authentication



## create_keda_trigger_authentication

> models::KedaTriggerAuthenticationResponse create_keda_trigger_authentication(organization_id, keda_trigger_authentication_request)
create Keda Trigger Authentications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**keda_trigger_authentication_request** | [**KedaTriggerAuthenticationRequest**](KedaTriggerAuthenticationRequest.md) |  | [required] |

### Return type

[**models::KedaTriggerAuthenticationResponse**](KedaTriggerAuthenticationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_keda_trigger_authentication

> delete_keda_trigger_authentication(organization_id, trigger_authentication_id)
Delete a KEDA trigger authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**trigger_authentication_id** | **String** | KEDA triggerAuthentication ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_keda_trigger_authentication

> models::KedaTriggerAuthenticationResponse get_keda_trigger_authentication(organization_id, trigger_authentication_id)
Get a KEDA trigger authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**trigger_authentication_id** | **String** | KEDA triggerAuthentication ID | [required] |

### Return type

[**models::KedaTriggerAuthenticationResponse**](KedaTriggerAuthenticationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_keda_trigger_authentications

> models::KedaTriggerAuthenticationResponseList list_keda_trigger_authentications(organization_id)
list Keda TriggerAuthentications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::KedaTriggerAuthenticationResponseList**](KedaTriggerAuthenticationResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_keda_trigger_authentication

> models::KedaTriggerAuthenticationResponse update_keda_trigger_authentication(organization_id, trigger_authentication_id, keda_trigger_authentication_request)
Update a KEDA trigger authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**trigger_authentication_id** | **String** | KEDA triggerAuthentication ID | [required] |
**keda_trigger_authentication_request** | Option<[**KedaTriggerAuthenticationRequest**](KedaTriggerAuthenticationRequest.md)> |  |  |

### Return type

[**models::KedaTriggerAuthenticationResponse**](KedaTriggerAuthenticationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

