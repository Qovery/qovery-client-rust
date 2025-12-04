# \OrganizationEventApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_organization_event_targets**](OrganizationEventApi.md#get_organization_event_targets) | **GET** /organization/{organizationId}/targets | Get available event targets to filter events
[**get_organization_events**](OrganizationEventApi.md#get_organization_events) | **GET** /organization/{organizationId}/events | Get all events inside the organization



## get_organization_event_targets

> models::OrganizationEventTargetResponseList get_organization_event_targets(organization_id, from_timestamp, to_timestamp, event_type, target_type, triggered_by, origin, project_id, environment_id, target_level_to_fetch)
Get available event targets to filter events

Get available event targets to filter events

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**from_timestamp** | Option<**String**> | Display targets available since this timestamp.   A range of date can be specified by using `from-timestamp` with `to-timestamp` The format is a timestamp with nano precision  |  |
**to_timestamp** | Option<**String**> | Display targets triggered before this timestamp.   A range of date can be specified by using `to-timestamp` with `from-timestamp` The format is a timestamp with nano precision  |  |
**event_type** | Option<[**OrganizationEventType**](.md)> |  |  |
**target_type** | Option<[**OrganizationEventTargetType**](.md)> |  |  |
**triggered_by** | Option<**String**> | Information about the owner of the event (user name / apitoken / automatic action) |  |
**origin** | Option<[**OrganizationEventOrigin**](.md)> |  |  |
**project_id** | Option<**uuid::Uuid**> | Mandatory when requesting an environment or a service |  |
**environment_id** | Option<**uuid::Uuid**> | Mandatory when requesting a service |  |
**target_level_to_fetch** | Option<[**OrganizationEventTargetLevel**](.md)> | Used only to retrieve projects or environments linked to service typed events |  |

### Return type

[**models::OrganizationEventTargetResponseList**](OrganizationEventTargetResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_events

> models::OrganizationEventResponseList get_organization_events(organization_id, page_size, from_timestamp, to_timestamp, continue_token, step_back_token, event_type, target_type, target_id, sub_target_type, triggered_by, origin)
Get all events inside the organization

Get all events inside the organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**page_size** | Option<**f64**> | The number of events to display in the current page |  |[default to 10]
**from_timestamp** | Option<**String**> | Display events triggered since this timestamp.   A range of date can be specified by using `from-timestamp` with `to-timestamp` The format is a timestamp with nano precision  |  |
**to_timestamp** | Option<**String**> | Display events triggered before this timestamp.   A range of date can be specified by using `to-timestamp` with `from-timestamp` The format is a timestamp with nano precision  |  |
**continue_token** | Option<**String**> | Token used to fetch the next page results The format is a timestamp with nano precision  |  |
**step_back_token** | Option<**String**> | Token used to fetch the previous page results The format is a timestamp with nano precision  |  |
**event_type** | Option<[**OrganizationEventType**](.md)> |  |  |
**target_type** | Option<[**OrganizationEventTargetType**](.md)> |  |  |
**target_id** | Option<**uuid::Uuid**> | The target resource id to search.   Must be specified with the corresponding `target_type`  |  |
**sub_target_type** | Option<[**OrganizationEventSubTargetType**](.md)> |  |  |
**triggered_by** | Option<**String**> | Information about the owner of the event (user name / apitoken / automatic action) |  |
**origin** | Option<[**OrganizationEventOrigin**](.md)> |  |  |

### Return type

[**models::OrganizationEventResponseList**](OrganizationEventResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

