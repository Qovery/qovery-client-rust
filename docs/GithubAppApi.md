# \GithubAppApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**organization_github_app_connect**](GithubAppApi.md#organization_github_app_connect) | **POST** /organization/{organizationId}/github/connect | Connect a github account to an organization
[**organization_github_app_disconnect**](GithubAppApi.md#organization_github_app_disconnect) | **DELETE** /organization/{organizationId}/github/disconnect | Disconnect a github account from an organization



## organization_github_app_connect

> organization_github_app_connect(organization_id, organization_github_app_connect_request)
Connect a github account to an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**organization_github_app_connect_request** | Option<[**OrganizationGithubAppConnectRequest**](OrganizationGithubAppConnectRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organization_github_app_disconnect

> organization_github_app_disconnect(organization_id, force)
Disconnect a github account from an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**force** | Option<**bool**> | Indicates if the github app should be disconnected despite github applications linked to organization |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

