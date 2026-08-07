# \OrganizationAgentApiTokenApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization_agent_api_token**](OrganizationAgentApiTokenApi.md#create_organization_agent_api_token) | **POST** /organization/{organizationId}/agentApiToken | Create an organization agent api token
[**delete_organization_agent_api_token**](OrganizationAgentApiTokenApi.md#delete_organization_agent_api_token) | **DELETE** /organization/{organizationId}/agentApiToken/{agentApiTokenId} | Delete organization agent api token
[**list_organization_agent_api_tokens**](OrganizationAgentApiTokenApi.md#list_organization_agent_api_tokens) | **GET** /organization/{organizationId}/agentApiToken | List organization agent api tokens



## create_organization_agent_api_token

> models::OrganizationAgentApiTokenCreate create_organization_agent_api_token(organization_id, organization_agent_api_token_create_request)
Create an organization agent api token

Create an agent api token, intended for autonomous agents calling the Qovery API.  Unlike a regular api token, an agent api token carries **no organization role**. The Open Policy Agent (rego) policy attached to it is evaluated on every request and is the only thing constraining what the token can do: a policy that allows everything grants full organization admin access. For that reason only an organization owner or admin can create one.  The generated token value is returned only in this response and cannot be retrieved afterwards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**organization_agent_api_token_create_request** | Option<[**OrganizationAgentApiTokenCreateRequest**](OrganizationAgentApiTokenCreateRequest.md)> |  |  |

### Return type

[**models::OrganizationAgentApiTokenCreate**](OrganizationAgentApiTokenCreate.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_agent_api_token

> delete_organization_agent_api_token(organization_id, agent_api_token_id)
Delete organization agent api token

Delete organization agent api token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**agent_api_token_id** | **uuid::Uuid** | Organization Agent Api Token ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_agent_api_tokens

> models::OrganizationAgentApiTokenResponseList list_organization_agent_api_tokens(organization_id)
List organization agent api tokens

List the agent api tokens of the organization, each with the Open Policy Agent (rego) policy it carries. The token value itself is never returned here, it is only shown once at creation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::OrganizationAgentApiTokenResponseList**](OrganizationAgentApiTokenResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

