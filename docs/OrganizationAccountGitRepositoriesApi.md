# \OrganizationAccountGitRepositoriesApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_organization_bitbucket_repositories**](OrganizationAccountGitRepositoriesApi.md#get_organization_bitbucket_repositories) | **GET** /organization/{organizationId}/account/bitbucket/repository | Get bitbucket repositories of the connected user
[**get_organization_bitbucket_repository_branches**](OrganizationAccountGitRepositoriesApi.md#get_organization_bitbucket_repository_branches) | **GET** /organization/{organizationId}/account/bitbucket/repository/branch | Get bitbucket branches of the specified repository
[**get_organization_git_provider_account**](OrganizationAccountGitRepositoriesApi.md#get_organization_git_provider_account) | **GET** /organization/{organizationId}/account/gitAuthProvider | Get git provider accounts
[**get_organization_github_repositories**](OrganizationAccountGitRepositoriesApi.md#get_organization_github_repositories) | **GET** /organization/{organizationId}/account/github/repository | Get github repositories of the connected user
[**get_organization_github_repository_branches**](OrganizationAccountGitRepositoriesApi.md#get_organization_github_repository_branches) | **GET** /organization/{organizationId}/account/github/repository/branch | Get github branches of the specified repository
[**get_organization_gitlab_repositories**](OrganizationAccountGitRepositoriesApi.md#get_organization_gitlab_repositories) | **GET** /organization/{organizationId}/account/gitlab/repository | Get gitlab repositories of the connected user
[**get_organization_gitlab_repository_branches**](OrganizationAccountGitRepositoriesApi.md#get_organization_gitlab_repository_branches) | **GET** /organization/{organizationId}/account/gitlab/repository/branch | Get gitlab branches of the specified repository



## get_organization_bitbucket_repositories

> models::GitRepositoryResponseList get_organization_bitbucket_repositories(organization_id, git_token_id)
Get bitbucket repositories of the connected user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**git_token_id** | Option<**uuid::Uuid**> | The git token id that must be used for the application |  |

### Return type

[**models::GitRepositoryResponseList**](GitRepositoryResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_bitbucket_repository_branches

> models::GitRepositoryBranchResponseList get_organization_bitbucket_repository_branches(organization_id, name, git_token_id)
Get bitbucket branches of the specified repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**name** | Option<**String**> | The name of the repository where to retrieve the branches |  |
**git_token_id** | Option<**uuid::Uuid**> | The git token id that must be used for the application |  |

### Return type

[**models::GitRepositoryBranchResponseList**](GitRepositoryBranchResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_git_provider_account

> models::GitAuthProviderResponseList get_organization_git_provider_account(organization_id)
Get git provider accounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::GitAuthProviderResponseList**](GitAuthProviderResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_github_repositories

> models::GitRepositoryResponseList get_organization_github_repositories(organization_id, git_token_id)
Get github repositories of the connected user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**git_token_id** | Option<**uuid::Uuid**> | The git token id that must be used for the application |  |

### Return type

[**models::GitRepositoryResponseList**](GitRepositoryResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_github_repository_branches

> models::GitRepositoryBranchResponseList get_organization_github_repository_branches(organization_id, name, git_token_id)
Get github branches of the specified repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**name** | Option<**String**> | The name of the repository where to retrieve the branches |  |
**git_token_id** | Option<**uuid::Uuid**> | The git token id that must be used for the application |  |

### Return type

[**models::GitRepositoryBranchResponseList**](GitRepositoryBranchResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_gitlab_repositories

> models::GitRepositoryResponseList get_organization_gitlab_repositories(organization_id, git_token_id)
Get gitlab repositories of the connected user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**git_token_id** | Option<**uuid::Uuid**> | The git token id that must be used for the application |  |

### Return type

[**models::GitRepositoryResponseList**](GitRepositoryResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_gitlab_repository_branches

> models::GitRepositoryBranchResponseList get_organization_gitlab_repository_branches(organization_id, name, git_token_id)
Get gitlab branches of the specified repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**name** | Option<**String**> | The name of the repository to retrieve the branches |  |
**git_token_id** | Option<**uuid::Uuid**> | The git token id that must be used for the application |  |

### Return type

[**models::GitRepositoryBranchResponseList**](GitRepositoryBranchResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

