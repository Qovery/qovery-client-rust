# \OrganizationCustomDomainApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_organization_custom_domain**](OrganizationCustomDomainApi.md#list_organization_custom_domain) | **GET** /organization/{organizationId}/customDomain | List organization custom domains



## list_organization_custom_domain

> models::OrganizationCustomDomainPaginatedResponseList list_organization_custom_domain(organization_id, project_id, environment_id, search, page, page_size)
List organization custom domains

List the custom domains of every environment of the organization. Only organization admins can call it. Each result is one domain name of one environment, with every route that serves it: a domain shared by several services of an environment is a single result. Results are sorted by project name, environment name, environment id, then domain. A page never splits a domain. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**project_id** | Option<**uuid::Uuid**> | Only list the domains of this project. A project of another organization gives an empty list. |  |
**environment_id** | Option<**uuid::Uuid**> | Only list the domains of this environment. An environment of another organization gives an empty list. |  |
**search** | Option<**String**> | Only list the domains whose name contains this text, ignoring case. The text is trimmed and matched literally, with no wildcard. A blank value does not filter. |  |
**page** | Option<**i32**> | The page to return, starting at 1. A page after the last one has no results. |  |[default to 1]
**page_size** | Option<**i32**> | The number of domains per page. |  |[default to 20]

### Return type

[**models::OrganizationCustomDomainPaginatedResponseList**](OrganizationCustomDomainPaginatedResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

