# \ContainerRegistriesApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_container_registry**](ContainerRegistriesApi.md#create_container_registry) | **POST** /organization/{organizationId}/containerRegistry | Create a container registry
[**delete_container_registry**](ContainerRegistriesApi.md#delete_container_registry) | **DELETE** /organization/{organizationId}/containerRegistry/{containerRegistryId} | Delete a container registry
[**edit_container_registry**](ContainerRegistriesApi.md#edit_container_registry) | **PUT** /organization/{organizationId}/containerRegistry/{containerRegistryId} | Edit a container registry
[**get_container_registry**](ContainerRegistriesApi.md#get_container_registry) | **GET** /organization/{organizationId}/containerRegistry/{containerRegistryId} | Get a container registry
[**get_container_versions**](ContainerRegistriesApi.md#get_container_versions) | **GET** /organization/{organizationId}/containerRegistry/{containerRegistryId}/images | List image version for a container registry
[**list_available_container_registry**](ContainerRegistriesApi.md#list_available_container_registry) | **GET** /availableContainerRegistry | List supported container registries
[**list_container_registry**](ContainerRegistriesApi.md#list_container_registry) | **GET** /organization/{organizationId}/containerRegistry | List organization container registries



## create_container_registry

> models::ContainerRegistryResponse create_container_registry(organization_id, container_registry_request)
Create a container registry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**container_registry_request** | Option<[**ContainerRegistryRequest**](ContainerRegistryRequest.md)> |  |  |

### Return type

[**models::ContainerRegistryResponse**](ContainerRegistryResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_container_registry

> delete_container_registry(organization_id, container_registry_id)
Delete a container registry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**container_registry_id** | **uuid::Uuid** | Container Registry ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_container_registry

> models::ContainerRegistryResponse edit_container_registry(organization_id, container_registry_id, container_registry_request)
Edit a container registry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**container_registry_id** | **uuid::Uuid** | Container Registry ID | [required] |
**container_registry_request** | Option<[**ContainerRegistryRequest**](ContainerRegistryRequest.md)> |  |  |

### Return type

[**models::ContainerRegistryResponse**](ContainerRegistryResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_container_registry

> models::ContainerRegistryResponse get_container_registry(organization_id, container_registry_id)
Get a container registry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**container_registry_id** | **uuid::Uuid** | Container Registry ID | [required] |

### Return type

[**models::ContainerRegistryResponse**](ContainerRegistryResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_container_versions

> models::ContainerVersionResponseList get_container_versions(organization_id, container_registry_id, image_name, search)
List image version for a container registry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**container_registry_id** | **uuid::Uuid** | Container Registry ID | [required] |
**image_name** | Option<**String**> | container image name |  |
**search** | Option<**String**> | partial container image name to search |  |

### Return type

[**models::ContainerVersionResponseList**](ContainerVersionResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_available_container_registry

> models::AvailableContainerRegistryResponseList list_available_container_registry()
List supported container registries

List supported container registries by Qovery and get the mandatory authentification configuration.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AvailableContainerRegistryResponseList**](AvailableContainerRegistryResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_container_registry

> models::ContainerRegistryResponseList list_container_registry(organization_id)
List organization container registries

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::ContainerRegistryResponseList**](ContainerRegistryResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

