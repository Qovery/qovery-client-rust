# \OrganizationMainCallsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_git_token**](OrganizationMainCallsApi.md#create_git_token) | **POST** /organization/{organizationId}/gitToken | Create a git token
[**create_organization**](OrganizationMainCallsApi.md#create_organization) | **POST** /organization | Create an organization
[**delete_git_token**](OrganizationMainCallsApi.md#delete_git_token) | **DELETE** /organization/{organizationId}/gitToken/{gitTokenId} | Delete a git token
[**delete_organization**](OrganizationMainCallsApi.md#delete_organization) | **DELETE** /organization/{organizationId} | Delete an organization
[**edit_git_token**](OrganizationMainCallsApi.md#edit_git_token) | **PUT** /organization/{organizationId}/gitToken/{gitTokenId} | Edit a git token
[**edit_organization**](OrganizationMainCallsApi.md#edit_organization) | **PUT** /organization/{organizationId} | Edit an organization
[**get_container_registry_associated_services**](OrganizationMainCallsApi.md#get_container_registry_associated_services) | **GET** /organization/{organizationId}/containerRegistry/{containerRegistryId}/associatedServices | Get organization container registry associated services
[**get_git_token_associated_services**](OrganizationMainCallsApi.md#get_git_token_associated_services) | **GET** /organization/{organizationId}/gitToken/{gitTokenId}/associatedServices | Get organization git token associated services
[**get_helm_repository_associated_services**](OrganizationMainCallsApi.md#get_helm_repository_associated_services) | **GET** /organization/{organizationId}/helmRepository/{helmRepositoryId}/associatedServices | Get organization helm repository associated services
[**get_organization**](OrganizationMainCallsApi.md#get_organization) | **GET** /organization/{organizationId} | Get organization by ID
[**get_organization_git_token**](OrganizationMainCallsApi.md#get_organization_git_token) | **GET** /organization/{organizationId}/gitToken/{gitTokenId} | Get organization git token
[**list_organization**](OrganizationMainCallsApi.md#list_organization) | **GET** /organization | List user organizations
[**list_organization_available_roles**](OrganizationMainCallsApi.md#list_organization_available_roles) | **GET** /organization/{organizationId}/availableRole | List organization available roles
[**list_organization_credentials**](OrganizationMainCallsApi.md#list_organization_credentials) | **GET** /organization/{organizationId}/credentials | List credentials of an organization and their associated clusters
[**list_organization_git_tokens**](OrganizationMainCallsApi.md#list_organization_git_tokens) | **GET** /organization/{organizationId}/gitToken | List organization git tokens
[**list_services_by_organization_id**](OrganizationMainCallsApi.md#list_services_by_organization_id) | **GET** /organization/{organizationId}/services | List Services By OrganizationId
[**list_tf_vars_files_from_git_repo**](OrganizationMainCallsApi.md#list_tf_vars_files_from_git_repo) | **POST** /organization/{organizationId}/listTfVarsFilesFromGitRepo | List Terraform tfvars files from Git repository
[**parse_terraform_variables_from_git_repo**](OrganizationMainCallsApi.md#parse_terraform_variables_from_git_repo) | **POST** /organization/{organizationId}/parseTerraformVariablesFromGitRepo | Parse Terraform variables from Git repository



## create_git_token

> models::GitTokenResponse create_git_token(organization_id, git_token_request)
Create a git token

Create a new git token to be used as a git provider by a service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**git_token_request** | Option<[**GitTokenRequest**](GitTokenRequest.md)> |  |  |

### Return type

[**models::GitTokenResponse**](GitTokenResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_organization

> models::Organization create_organization(organization_request)
Create an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_request** | Option<[**OrganizationRequest**](OrganizationRequest.md)> |  |  |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_git_token

> delete_git_token(organization_id, git_token_id)
Delete a git token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**git_token_id** | **uuid::Uuid** | Git Token ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization

> delete_organization(organization_id)
Delete an organization

To delete an organization you must have the admin permission

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_git_token

> models::GitTokenResponse edit_git_token(organization_id, git_token_id, git_token_request)
Edit a git token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**git_token_id** | **uuid::Uuid** | Git Token ID | [required] |
**git_token_request** | Option<[**GitTokenRequest**](GitTokenRequest.md)> |  |  |

### Return type

[**models::GitTokenResponse**](GitTokenResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_organization

> models::Organization edit_organization(organization_id, organization_edit_request)
Edit an organization

To edit an organization you must have the admin permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**organization_edit_request** | Option<[**OrganizationEditRequest**](OrganizationEditRequest.md)> |  |  |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_container_registry_associated_services

> models::ContainerRegistryAssociatedServicesResponseList get_container_registry_associated_services(organization_id, container_registry_id)
Get organization container registry associated services

Get organization container registry associated services

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |
**container_registry_id** | **String** |  | [required] |

### Return type

[**models::ContainerRegistryAssociatedServicesResponseList**](ContainerRegistryAssociatedServicesResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_git_token_associated_services

> models::GitTokenAssociatedServicesResponseList get_git_token_associated_services(organization_id, git_token_id)
Get organization git token associated services

Get organization git tokens associated services

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**git_token_id** | **uuid::Uuid** | Git Token ID | [required] |

### Return type

[**models::GitTokenAssociatedServicesResponseList**](GitTokenAssociatedServicesResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_helm_repository_associated_services

> models::HelmRepositoryAssociatedServicesResponseList get_helm_repository_associated_services(organization_id, helm_repository_id)
Get organization helm repository associated services

Get organization helm repository associated services

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |
**helm_repository_id** | **String** |  | [required] |

### Return type

[**models::HelmRepositoryAssociatedServicesResponseList**](HelmRepositoryAssociatedServicesResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization

> models::Organization get_organization(organization_id)
Get organization by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_git_token

> models::GitTokenResponse get_organization_git_token(organization_id, git_token_id)
Get organization git token

Get organization git token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**git_token_id** | **uuid::Uuid** | Git Token ID | [required] |

### Return type

[**models::GitTokenResponse**](GitTokenResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization

> models::OrganizationResponseList list_organization()
List user organizations

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OrganizationResponseList**](OrganizationResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_available_roles

> models::OrganizationAvailableRoleList list_organization_available_roles(organization_id)
List organization available roles

List organization available roles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::OrganizationAvailableRoleList**](OrganizationAvailableRoleList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_credentials

> models::OrganizationCrendentialsResponseList list_organization_credentials(organization_id)
List credentials of an organization and their associated clusters

List credentials of an organization and their associated clusters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |

### Return type

[**models::OrganizationCrendentialsResponseList**](OrganizationCrendentialsResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_git_tokens

> models::GitTokenResponseList list_organization_git_tokens(organization_id)
List organization git tokens

List organization git tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::GitTokenResponseList**](GitTokenResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_services_by_organization_id

> models::ListServicesByOrganizationId200Response list_services_by_organization_id(organization_id, project_id, environment_id, cluster_id)
List Services By OrganizationId

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |
**project_id** | Option<**String**> |  |  |
**environment_id** | Option<**String**> |  |  |
**cluster_id** | Option<**String**> |  |  |

### Return type

[**models::ListServicesByOrganizationId200Response**](listServicesByOrganizationId_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tf_vars_files_from_git_repo

> models::ListTfVarsFilesFromGitRepo200Response list_tf_vars_files_from_git_repo(organization_id, tf_vars_list_request)
List Terraform tfvars files from Git repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**tf_vars_list_request** | [**TfVarsListRequest**](TfVarsListRequest.md) |  | [required] |

### Return type

[**models::ListTfVarsFilesFromGitRepo200Response**](listTfVarsFilesFromGitRepo_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## parse_terraform_variables_from_git_repo

> models::ParseTerraformVariablesFromGitRepo200Response parse_terraform_variables_from_git_repo(organization_id, terraform_variable_parsing_request)
Parse Terraform variables from Git repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**terraform_variable_parsing_request** | [**TerraformVariableParsingRequest**](TerraformVariableParsingRequest.md) |  | [required] |

### Return type

[**models::ParseTerraformVariablesFromGitRepo200Response**](parseTerraformVariablesFromGitRepo_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

