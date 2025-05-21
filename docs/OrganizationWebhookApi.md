# \OrganizationWebhookApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization_webhook**](OrganizationWebhookApi.md#create_organization_webhook) | **POST** /organization/{organizationId}/webhook | Create an organization webhook
[**delete_organization_webhook**](OrganizationWebhookApi.md#delete_organization_webhook) | **DELETE** /organization/{organizationId}/webhook/{webhookId} | Delete organization webhook
[**edit_organization_webhook**](OrganizationWebhookApi.md#edit_organization_webhook) | **PUT** /organization/{organizationId}/webhook/{webhookId} | Edit an organization webhook
[**get_organization_webhook**](OrganizationWebhookApi.md#get_organization_webhook) | **GET** /organization/{organizationId}/webhook/{webhookId} | Get an Organization webhook
[**list_organization_web_hooks**](OrganizationWebhookApi.md#list_organization_web_hooks) | **GET** /organization/{organizationId}/webhook | List organization webhooks



## create_organization_webhook

> models::OrganizationWebhookCreateResponse create_organization_webhook(organization_id, organization_webhook_create_request)
Create an organization webhook

Create an organization webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**organization_webhook_create_request** | Option<[**OrganizationWebhookCreateRequest**](OrganizationWebhookCreateRequest.md)> |  |  |

### Return type

[**models::OrganizationWebhookCreateResponse**](OrganizationWebhookCreateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_webhook

> delete_organization_webhook(organization_id, webhook_id)
Delete organization webhook

Delete organization webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**webhook_id** | **uuid::Uuid** | Webhook ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_organization_webhook

> models::OrganizationWebhookCreateResponse edit_organization_webhook(organization_id, webhook_id, organization_webhook_create_request)
Edit an organization webhook

Edit an organization webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**webhook_id** | **uuid::Uuid** | Webhook ID | [required] |
**organization_webhook_create_request** | Option<[**OrganizationWebhookCreateRequest**](OrganizationWebhookCreateRequest.md)> |  |  |

### Return type

[**models::OrganizationWebhookCreateResponse**](OrganizationWebhookCreateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_webhook

> models::OrganizationWebhookResponse get_organization_webhook(organization_id, webhook_id)
Get an Organization webhook

Get an Organization webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**webhook_id** | **uuid::Uuid** | Webhook ID | [required] |

### Return type

[**models::OrganizationWebhookResponse**](OrganizationWebhookResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_web_hooks

> models::OrganizationWebhookResponseList list_organization_web_hooks(organization_id)
List organization webhooks

List organization webhooks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::OrganizationWebhookResponseList**](OrganizationWebhookResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

