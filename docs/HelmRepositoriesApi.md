# \HelmRepositoriesApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_helm_repository**](HelmRepositoriesApi.md#create_helm_repository) | **POST** /organization/{organizationId}/helmRepository | Create a helm repository
[**delete_helm_repository**](HelmRepositoriesApi.md#delete_helm_repository) | **DELETE** /organization/{organizationId}/helmRepository/{helmRepositoryId} | Delete a helm repository
[**edit_helm_repository**](HelmRepositoriesApi.md#edit_helm_repository) | **PUT** /organization/{organizationId}/helmRepository/{helmRepositoryId} | Edit a helm repository
[**get_helm_charts**](HelmRepositoriesApi.md#get_helm_charts) | **GET** /organization/{organizationId}/helmRepository/{helmRepositoryId}/charts | List helm charts contained inside the repository
[**get_helm_repository**](HelmRepositoriesApi.md#get_helm_repository) | **GET** /organization/{organizationId}/helmRepository/{helmRepositoryId} | Get a helm repository
[**list_available_helm_repository**](HelmRepositoriesApi.md#list_available_helm_repository) | **GET** /availableHelmRepository | List supported helm repository
[**list_helm_repository**](HelmRepositoriesApi.md#list_helm_repository) | **GET** /organization/{organizationId}/helmRepository | List organization helm repositories



## create_helm_repository

> models::HelmRepositoryResponse create_helm_repository(organization_id, helm_repository_request)
Create a helm repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**helm_repository_request** | Option<[**HelmRepositoryRequest**](HelmRepositoryRequest.md)> |  |  |

### Return type

[**models::HelmRepositoryResponse**](HelmRepositoryResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_helm_repository

> delete_helm_repository(organization_id, helm_repository_id)
Delete a helm repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**helm_repository_id** | **uuid::Uuid** | Helm chart repository ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_helm_repository

> models::HelmRepositoryResponse edit_helm_repository(organization_id, helm_repository_id, helm_repository_request)
Edit a helm repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**helm_repository_id** | **uuid::Uuid** | Helm chart repository ID | [required] |
**helm_repository_request** | Option<[**HelmRepositoryRequest**](HelmRepositoryRequest.md)> |  |  |

### Return type

[**models::HelmRepositoryResponse**](HelmRepositoryResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_helm_charts

> models::HelmVersionResponseList get_helm_charts(organization_id, helm_repository_id, chart_name)
List helm charts contained inside the repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**helm_repository_id** | **uuid::Uuid** | Helm chart repository ID | [required] |
**chart_name** | Option<**String**> | Helm chart name to filter the result on |  |

### Return type

[**models::HelmVersionResponseList**](HelmVersionResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_helm_repository

> models::HelmRepositoryResponse get_helm_repository(organization_id, helm_repository_id)
Get a helm repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**helm_repository_id** | **uuid::Uuid** | Helm chart repository ID | [required] |

### Return type

[**models::HelmRepositoryResponse**](HelmRepositoryResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_available_helm_repository

> models::AvailableHelmRepositoryResponseList list_available_helm_repository()
List supported helm repository

List supported helm repository by Qovery and get the mandatory authentification configuration.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AvailableHelmRepositoryResponseList**](AvailableHelmRepositoryResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_helm_repository

> models::HelmRepositoryResponseList list_helm_repository(organization_id)
List organization helm repositories

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::HelmRepositoryResponseList**](HelmRepositoryResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

