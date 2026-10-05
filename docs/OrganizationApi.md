# \OrganizationApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**track_skill_call**](OrganizationApi.md#track_skill_call) | **POST** /organization/{organizationId}/skill-tracking | Track a skill call



## track_skill_call

> track_skill_call(organization_id, skill_tracking_request, user_agent)
Track a skill call

Track a skill call

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**skill_tracking_request** | [**SkillTrackingRequest**](SkillTrackingRequest.md) |  | [required] |
**user_agent** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

