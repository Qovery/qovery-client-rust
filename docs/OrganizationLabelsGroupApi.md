# \OrganizationLabelsGroupApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization_labels_group**](OrganizationLabelsGroupApi.md#create_organization_labels_group) | **POST** /organization/{organizationId}/labelsGroups | Create an organization labels group
[**delete_organization_labels_group**](OrganizationLabelsGroupApi.md#delete_organization_labels_group) | **DELETE** /organization/{organizationId}/labelsGroups/{labelsGroupId} | Delete organization labels group
[**edit_organization_labels_group**](OrganizationLabelsGroupApi.md#edit_organization_labels_group) | **PUT** /organization/{organizationId}/labelsGroups/{labelsGroupId} | Edit organization labels group
[**get_organization_labels_group_associated_items**](OrganizationLabelsGroupApi.md#get_organization_labels_group_associated_items) | **GET** /organization/{organizationId}/labelsGroups/{labelsGroupId}/associatedItems | Get organization labels group associated items
[**get_organization_labelss_group**](OrganizationLabelsGroupApi.md#get_organization_labelss_group) | **GET** /organization/{organizationId}/labelsGroups/{labelsGroupId} | Get organization labels group
[**list_organization_labels_group**](OrganizationLabelsGroupApi.md#list_organization_labels_group) | **GET** /organization/{organizationId}/labelsGroups | List organization labels group



## create_organization_labels_group

> models::OrganizationLabelsGroupResponse create_organization_labels_group(organization_id, organization_labels_group_create_request)
Create an organization labels group

Create an organization labels group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**organization_labels_group_create_request** | Option<[**OrganizationLabelsGroupCreateRequest**](OrganizationLabelsGroupCreateRequest.md)> |  |  |

### Return type

[**models::OrganizationLabelsGroupResponse**](OrganizationLabelsGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_labels_group

> delete_organization_labels_group(organization_id, labels_group_id)
Delete organization labels group

Delete organization labels group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**labels_group_id** | **uuid::Uuid** | Organization labels group ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_organization_labels_group

> models::OrganizationLabelsGroupResponse edit_organization_labels_group(organization_id, labels_group_id, organization_labels_group_create_request)
Edit organization labels group

Edit organization labels group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**labels_group_id** | **uuid::Uuid** | Organization labels group ID | [required] |
**organization_labels_group_create_request** | Option<[**OrganizationLabelsGroupCreateRequest**](OrganizationLabelsGroupCreateRequest.md)> |  |  |

### Return type

[**models::OrganizationLabelsGroupResponse**](OrganizationLabelsGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_labels_group_associated_items

> models::OrganizationLabelsGroupAssociatedItemsResponseList get_organization_labels_group_associated_items(organization_id, labels_group_id)
Get organization labels group associated items

Get organization labels group associated items

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**labels_group_id** | **uuid::Uuid** | Organization labels group ID | [required] |

### Return type

[**models::OrganizationLabelsGroupAssociatedItemsResponseList**](OrganizationLabelsGroupAssociatedItemsResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_labelss_group

> models::OrganizationLabelsGroupResponse get_organization_labelss_group(organization_id, labels_group_id)
Get organization labels group

Get organization labels group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**labels_group_id** | **uuid::Uuid** | Organization labels group ID | [required] |

### Return type

[**models::OrganizationLabelsGroupResponse**](OrganizationLabelsGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_labels_group

> models::ListOrganizationLabelsGroup200Response list_organization_labels_group(organization_id)
List organization labels group

List organization labels group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::ListOrganizationLabelsGroup200Response**](listOrganizationLabelsGroup_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

