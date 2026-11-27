# \OrganizationCustomDomainApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_delete_organization_custom_domain**](OrganizationCustomDomainApi.md#bulk_delete_organization_custom_domain) | **POST** /organization/{organizationId}/customDomain/bulkDelete | Delete custom domains of several environments
[**check_organization_custom_domains**](OrganizationCustomDomainApi.md#check_organization_custom_domains) | **GET** /organization/{organizationId}/customDomain/check | Check the DNS of the organization's custom domains
[**list_organization_custom_domain**](OrganizationCustomDomainApi.md#list_organization_custom_domain) | **GET** /organization/{organizationId}/customDomain | List organization custom domains



## bulk_delete_organization_custom_domain

> models::CustomDomainDeletionResponse bulk_delete_organization_custom_domain(organization_id, custom_domain_bulk_delete_item_request)
Delete custom domains of several environments

Delete custom domains of several environments of the organization at once: all of them, or none. Each item deletes every row of the domain name in its environment, for every service routing it, with their routes. The caller needs the permission to manage each environment. An environment of another organization refuses the whole call with a 403, and nothing is deleted. Nothing is deployed: redeploy the services listed in `impacted_services` to apply the change. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**custom_domain_bulk_delete_item_request** | [**Vec<models::CustomDomainBulkDeleteItemRequest>**](CustomDomainBulkDeleteItemRequest.md) |  | [required] |

### Return type

[**models::CustomDomainDeletionResponse**](CustomDomainDeletionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_organization_custom_domains

> models::OrganizationCustomDomainCheckListResponse check_organization_custom_domains(organization_id, project_id, environment_id, search, page, page_size)
Check the DNS of the organization's custom domains

Check the DNS of the custom domains of a page of the organization domains list, in one call. Only organization admins can call it. The query parameters are the ones of `listOrganizationCustomDomain`: the same values select the same domains, in the same order. There is one result per domain name of one environment, and one check per service the domain routes to. A check that could not run (cluster unreachable, timeout, service never deployed) has a `null` `status`; the other checks are still returned. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**project_id** | Option<**uuid::Uuid**> | Only check the domains of this project. A project of another organization gives an empty list. |  |
**environment_id** | Option<**uuid::Uuid**> | Only check the domains of this environment. An environment of another organization gives an empty list. |  |
**search** | Option<**String**> | Only check the domains whose name contains this text, ignoring case. The text is trimmed and matched literally, with no wildcard. A blank value does not filter. |  |
**page** | Option<**i32**> | The page to check, starting at 1. A page after the last one has no results. |  |[default to 1]
**page_size** | Option<**i32**> | The number of domains per page. |  |[default to 20]

### Return type

[**models::OrganizationCustomDomainCheckListResponse**](OrganizationCustomDomainCheckListResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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

