# \OrganizationPolicyApiTokenApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization_policy_api_token**](OrganizationPolicyApiTokenApi.md#create_organization_policy_api_token) | **POST** /organization/{organizationId}/policyApiToken | Create an organization policy api token
[**delete_organization_policy_api_token**](OrganizationPolicyApiTokenApi.md#delete_organization_policy_api_token) | **DELETE** /organization/{organizationId}/policyApiToken/{policyApiTokenId} | Delete organization policy api token
[**list_organization_policy_api_tokens**](OrganizationPolicyApiTokenApi.md#list_organization_policy_api_tokens) | **GET** /organization/{organizationId}/policyApiToken | List organization policy api tokens



## create_organization_policy_api_token

> models::OrganizationPolicyApiTokenCreate create_organization_policy_api_token(organization_id, organization_policy_api_token_create_request)
Create an organization policy api token

Create a policy api token, intended for autonomous agents calling the Qovery API.  A policy api token carries two limits, and a request must pass both:  1. Its Open Policy Agent (rego) policy, evaluated on every request. A request the policy does not allow is refused with 401. 2. Its organization role, set with `role_id`. A request the policy allows then runs with that role's permissions, and a request the role does not permit is refused with 403.  Effective access is therefore the intersection of the policy and the role. When `role_id` is omitted, the token gets the organization admin role, so a policy that allows everything then grants full admin access. A token can never act as the Owner role.  Who can create one depends on the role requested: an organization owner or admin can create a token for any role except Owner; a DevOps member only for the DevOps, Viewer or Billing role or a custom role. Other callers are refused with 403.  A token cannot be edited: its policy, role and expiry are fixed at creation. To change them, create a new token and delete the old one. Deleting a custom role also deletes the policy api tokens that use it.  The generated token value is returned only in this response and cannot be retrieved afterwards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**organization_policy_api_token_create_request** | Option<[**OrganizationPolicyApiTokenCreateRequest**](OrganizationPolicyApiTokenCreateRequest.md)> |  |  |

### Return type

[**models::OrganizationPolicyApiTokenCreate**](OrganizationPolicyApiTokenCreate.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_policy_api_token

> delete_organization_policy_api_token(organization_id, policy_api_token_id)
Delete organization policy api token

Delete organization policy api token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**policy_api_token_id** | **uuid::Uuid** | Organization Policy Api Token ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_policy_api_tokens

> models::OrganizationPolicyApiTokenResponseList list_organization_policy_api_tokens(organization_id)
List organization policy api tokens

List the policy api tokens of the organization, each with the Open Policy Agent (rego) policy it carries. The token value itself is never returned here, it is only shown once at creation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::OrganizationPolicyApiTokenResponseList**](OrganizationPolicyApiTokenResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

