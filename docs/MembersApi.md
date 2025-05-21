# \MembersApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_invite_member**](MembersApi.md#delete_invite_member) | **DELETE** /organization/{organizationId}/inviteMember/{inviteId} | Remove an invited member
[**delete_member**](MembersApi.md#delete_member) | **DELETE** /organization/{organizationId}/member | Remove a member
[**edit_organization_member_role**](MembersApi.md#edit_organization_member_role) | **PUT** /organization/{organizationId}/member | Edit an organization member role
[**get_member_invitation**](MembersApi.md#get_member_invitation) | **GET** /organization/{organizationId}/inviteMember/{inviteId} | Get member invitation
[**get_organization_invited_members**](MembersApi.md#get_organization_invited_members) | **GET** /organization/{organizationId}/inviteMember | Get invited members
[**get_organization_members**](MembersApi.md#get_organization_members) | **GET** /organization/{organizationId}/member | Get organization members
[**post_accept_invite_member**](MembersApi.md#post_accept_invite_member) | **POST** /organization/{organizationId}/inviteMember/{inviteId} | Accept Invite in the organization
[**post_invite_member**](MembersApi.md#post_invite_member) | **POST** /organization/{organizationId}/inviteMember | Invite someone in the organization
[**post_organization_transfer_ownership**](MembersApi.md#post_organization_transfer_ownership) | **POST** /organization/{organizationId}/transferOwnership | Transfer organization ownership to another user



## delete_invite_member

> delete_invite_member(organization_id, invite_id)
Remove an invited member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**invite_id** | **uuid::Uuid** | Invite ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_member

> delete_member(organization_id, delete_member_request)
Remove a member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**delete_member_request** | Option<[**DeleteMemberRequest**](DeleteMemberRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_organization_member_role

> edit_organization_member_role(organization_id, member_role_update_request)
Edit an organization member role

Edit an organization member role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**member_role_update_request** | Option<[**MemberRoleUpdateRequest**](MemberRoleUpdateRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_member_invitation

> models::InviteMember get_member_invitation(organization_id, invite_id)
Get member invitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**invite_id** | **uuid::Uuid** | Invite ID | [required] |

### Return type

[**models::InviteMember**](InviteMember.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_invited_members

> models::InviteMemberResponseList get_organization_invited_members(organization_id)
Get invited members

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::InviteMemberResponseList**](InviteMemberResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_members

> models::MemberResponseList get_organization_members(organization_id)
Get organization members

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::MemberResponseList**](MemberResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_accept_invite_member

> models::InviteMember post_accept_invite_member(organization_id, invite_id)
Accept Invite in the organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**invite_id** | **uuid::Uuid** | Invite ID | [required] |

### Return type

[**models::InviteMember**](InviteMember.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_invite_member

> models::InviteMember post_invite_member(organization_id, invite_member_request)
Invite someone in the organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**invite_member_request** | Option<[**InviteMemberRequest**](InviteMemberRequest.md)> |  |  |

### Return type

[**models::InviteMember**](InviteMember.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_organization_transfer_ownership

> post_organization_transfer_ownership(organization_id, transfer_ownership_request)
Transfer organization ownership to another user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**transfer_ownership_request** | Option<[**TransferOwnershipRequest**](TransferOwnershipRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

