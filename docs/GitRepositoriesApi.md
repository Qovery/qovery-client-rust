# \GitRepositoriesApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_bitbucket_repositories**](GitRepositoriesApi.md#get_bitbucket_repositories) | **GET** /account/bitbucket/repository | Get bitbucket repositories of the connected user
[**get_bitbucket_repository_branches**](GitRepositoriesApi.md#get_bitbucket_repository_branches) | **GET** /account/bitbucket/repository/branch | Get bitbucket branches of the specified repository
[**get_git_provider_account**](GitRepositoriesApi.md#get_git_provider_account) | **GET** /account/gitAuthProvider | Get git provider accounts
[**get_github_repositories**](GitRepositoriesApi.md#get_github_repositories) | **GET** /account/github/repository | Get github repositories of the connected user
[**get_github_repository_branches**](GitRepositoriesApi.md#get_github_repository_branches) | **GET** /account/github/repository/branch | Get github branches of the specified repository
[**get_gitlab_repositories**](GitRepositoriesApi.md#get_gitlab_repositories) | **GET** /account/gitlab/repository | Get gitlab repositories of the connected user
[**get_gitlab_repository_branches**](GitRepositoriesApi.md#get_gitlab_repository_branches) | **GET** /account/gitlab/repository/branch | Get gitlab branches of the specified repository
[**list_directories_from_git_repository**](GitRepositoriesApi.md#list_directories_from_git_repository) | **POST** /organization/{organizationId}/listDirectoriesFromGitRepository | List directories from a git repository



## get_bitbucket_repositories

> models::GitRepositoryResponseList get_bitbucket_repositories()
Get bitbucket repositories of the connected user

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GitRepositoryResponseList**](GitRepositoryResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bitbucket_repository_branches

> models::GitRepositoryBranchResponseList get_bitbucket_repository_branches(name)
Get bitbucket branches of the specified repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the repository where to retrieve the branches |  |

### Return type

[**models::GitRepositoryBranchResponseList**](GitRepositoryBranchResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_git_provider_account

> models::GitAuthProviderResponseList get_git_provider_account()
Get git provider accounts

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GitAuthProviderResponseList**](GitAuthProviderResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_github_repositories

> models::GitRepositoryResponseList get_github_repositories()
Get github repositories of the connected user

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GitRepositoryResponseList**](GitRepositoryResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_github_repository_branches

> models::GitRepositoryBranchResponseList get_github_repository_branches(name)
Get github branches of the specified repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the repository where to retrieve the branches |  |

### Return type

[**models::GitRepositoryBranchResponseList**](GitRepositoryBranchResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gitlab_repositories

> models::GitRepositoryResponseList get_gitlab_repositories()
Get gitlab repositories of the connected user

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GitRepositoryResponseList**](GitRepositoryResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gitlab_repository_branches

> models::GitRepositoryBranchResponseList get_gitlab_repository_branches(name)
Get gitlab branches of the specified repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the repository to retrieve the branches |  |

### Return type

[**models::GitRepositoryBranchResponseList**](GitRepositoryBranchResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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

