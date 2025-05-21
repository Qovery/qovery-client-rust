# \OrganizationAnnotationsGroupApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization_annotations_group**](OrganizationAnnotationsGroupApi.md#create_organization_annotations_group) | **POST** /organization/{organizationId}/annotationsGroups | Create an organization annotations group
[**delete_organization_annotations_group**](OrganizationAnnotationsGroupApi.md#delete_organization_annotations_group) | **DELETE** /organization/{organizationId}/annotationsGroups/{annotationsGroupId} | Delete organization annotations group
[**edit_organization_annotations_group**](OrganizationAnnotationsGroupApi.md#edit_organization_annotations_group) | **PUT** /organization/{organizationId}/annotationsGroups/{annotationsGroupId} | Edit organization annotations group
[**get_organization_annotations_group**](OrganizationAnnotationsGroupApi.md#get_organization_annotations_group) | **GET** /organization/{organizationId}/annotationsGroups/{annotationsGroupId} | Get organization annotations group
[**get_organization_annotations_group_associated_items**](OrganizationAnnotationsGroupApi.md#get_organization_annotations_group_associated_items) | **GET** /organization/{organizationId}/annotationsGroups/{annotationsGroupId}/associatedItems | Get organization annotations group associated items
[**list_organization_annotations_group**](OrganizationAnnotationsGroupApi.md#list_organization_annotations_group) | **GET** /organization/{organizationId}/annotationsGroups | List organization annotations group



## create_organization_annotations_group

> models::OrganizationAnnotationsGroupResponse create_organization_annotations_group(organization_id, organization_annotations_group_create_request)
Create an organization annotations group

Create an organization annotations group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**organization_annotations_group_create_request** | Option<[**OrganizationAnnotationsGroupCreateRequest**](OrganizationAnnotationsGroupCreateRequest.md)> |  |  |

### Return type

[**models::OrganizationAnnotationsGroupResponse**](OrganizationAnnotationsGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_annotations_group

> delete_organization_annotations_group(organization_id, annotations_group_id)
Delete organization annotations group

Delete organization annotations group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**annotations_group_id** | **uuid::Uuid** | Organization annotations group ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_organization_annotations_group

> models::OrganizationAnnotationsGroupResponse edit_organization_annotations_group(organization_id, annotations_group_id, organization_annotations_group_create_request)
Edit organization annotations group

Edit organization annotations group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**annotations_group_id** | **uuid::Uuid** | Organization annotations group ID | [required] |
**organization_annotations_group_create_request** | Option<[**OrganizationAnnotationsGroupCreateRequest**](OrganizationAnnotationsGroupCreateRequest.md)> |  |  |

### Return type

[**models::OrganizationAnnotationsGroupResponse**](OrganizationAnnotationsGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_annotations_group

> models::OrganizationAnnotationsGroupResponse get_organization_annotations_group(organization_id, annotations_group_id)
Get organization annotations group

Get organization annotations group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**annotations_group_id** | **uuid::Uuid** | Organization annotations group ID | [required] |

### Return type

[**models::OrganizationAnnotationsGroupResponse**](OrganizationAnnotationsGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_annotations_group_associated_items

> models::OrganizationAnnotationsGroupAssociatedItemsResponseList get_organization_annotations_group_associated_items(organization_id, annotations_group_id)
Get organization annotations group associated items

Get organization annotations group associated items

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**annotations_group_id** | **uuid::Uuid** | Organization annotations group ID | [required] |

### Return type

[**models::OrganizationAnnotationsGroupAssociatedItemsResponseList**](OrganizationAnnotationsGroupAssociatedItemsResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_annotations_group

> models::ListOrganizationAnnotationsGroup200Response list_organization_annotations_group(organization_id)
List organization annotations group

List organization annotations group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::ListOrganizationAnnotationsGroup200Response**](listOrganizationAnnotationsGroup_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

