# \OrganizationCustomRoleApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization_custom_role**](OrganizationCustomRoleApi.md#create_organization_custom_role) | **POST** /organization/{organizationId}/customRole | Create an organization custom role
[**delete_organization_custom_role**](OrganizationCustomRoleApi.md#delete_organization_custom_role) | **DELETE** /organization/{organizationId}/customRole/{customRoleId} | Delete organization custom role
[**edit_organization_custom_role**](OrganizationCustomRoleApi.md#edit_organization_custom_role) | **PUT** /organization/{organizationId}/customRole/{customRoleId} | Edit an organization custom role
[**get_organization_custom_role**](OrganizationCustomRoleApi.md#get_organization_custom_role) | **GET** /organization/{organizationId}/customRole/{customRoleId} | Get an organization custom role 
[**list_organization_custom_roles**](OrganizationCustomRoleApi.md#list_organization_custom_roles) | **GET** /organization/{organizationId}/customRole | List organization custom roles



## create_organization_custom_role

> models::OrganizationCustomRole create_organization_custom_role(organization_id, organization_custom_role_create_request)
Create an organization custom role

Create an organization custom role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**organization_custom_role_create_request** | Option<[**OrganizationCustomRoleCreateRequest**](OrganizationCustomRoleCreateRequest.md)> |  |  |

### Return type

[**models::OrganizationCustomRole**](OrganizationCustomRole.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_custom_role

> delete_organization_custom_role(organization_id, custom_role_id)
Delete organization custom role

Delete organization custom role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**custom_role_id** | **uuid::Uuid** | Custom Role ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_organization_custom_role

> models::OrganizationCustomRole edit_organization_custom_role(organization_id, custom_role_id, organization_custom_role_update_request)
Edit an organization custom role

Edit an organization custom role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**custom_role_id** | **uuid::Uuid** | Custom Role ID | [required] |
**organization_custom_role_update_request** | Option<[**OrganizationCustomRoleUpdateRequest**](OrganizationCustomRoleUpdateRequest.md)> |  |  |

### Return type

[**models::OrganizationCustomRole**](OrganizationCustomRole.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_custom_role

> models::OrganizationCustomRole get_organization_custom_role(organization_id, custom_role_id)
Get an organization custom role 

Get an organization custom role 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**custom_role_id** | **uuid::Uuid** | Custom Role ID | [required] |

### Return type

[**models::OrganizationCustomRole**](OrganizationCustomRole.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_custom_roles

> models::OrganizationCustomRoleList list_organization_custom_roles(organization_id)
List organization custom roles

List organization custom roles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::OrganizationCustomRoleList**](OrganizationCustomRoleList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

