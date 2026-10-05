# \GitRepositoriesApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_directories_from_git_repository**](GitRepositoriesApi.md#list_directories_from_git_repository) | **POST** /organization/{organizationId}/listDirectoriesFromGitRepository | List directories from a git repository



## list_directories_from_git_repository

> models::ListDirectoriesFromGitRepository200Response list_directories_from_git_repository(organization_id, application_git_repository_request)
List directories from a git repository

List immediate subdirectories at a specified path in a git repository. This endpoint is used when creating Terraform services to help users browse and select the appropriate root path. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**application_git_repository_request** | Option<[**ApplicationGitRepositoryRequest**](ApplicationGitRepositoryRequest.md)> |  |  |

### Return type

[**models::ListDirectoriesFromGitRepository200Response**](listDirectoriesFromGitRepository_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

