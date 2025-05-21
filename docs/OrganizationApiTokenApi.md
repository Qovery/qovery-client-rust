# \OrganizationApiTokenApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization_api_token**](OrganizationApiTokenApi.md#create_organization_api_token) | **POST** /organization/{organizationId}/apiToken | Create an organization api token
[**delete_organization_api_token**](OrganizationApiTokenApi.md#delete_organization_api_token) | **DELETE** /organization/{organizationId}/apiToken/{apiTokenId} | Delete organization api token
[**list_organization_api_tokens**](OrganizationApiTokenApi.md#list_organization_api_tokens) | **GET** /organization/{organizationId}/apiToken | List organization api tokens



## create_organization_api_token

> models::OrganizationApiTokenCreate create_organization_api_token(organization_id, organization_api_token_create_request)
Create an organization api token

Create an organization api token. You can use the generated token to interact in a programmatic way with our API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**organization_api_token_create_request** | Option<[**OrganizationApiTokenCreateRequest**](OrganizationApiTokenCreateRequest.md)> |  |  |

### Return type

[**models::OrganizationApiTokenCreate**](OrganizationApiTokenCreate.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_api_token

> delete_organization_api_token(organization_id, api_token_id)
Delete organization api token

Delete organization api token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**api_token_id** | **uuid::Uuid** | Organization Api Token ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_api_tokens

> models::OrganizationApiTokenResponseList list_organization_api_tokens(organization_id)
List organization api tokens

List organization api tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::OrganizationApiTokenResponseList**](OrganizationApiTokenResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

