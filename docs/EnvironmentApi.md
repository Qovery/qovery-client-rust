# \EnvironmentApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_container_image**](EnvironmentApi.md#check_container_image) | **POST** /environment/{environmentId}/checkContainerImage | Check container image configuration is correct
[**check_dockerfile**](EnvironmentApi.md#check_dockerfile) | **POST** /environment/{environmentId}/checkDockerfile | Check dockerfile configuration is correct
[**check_git_file**](EnvironmentApi.md#check_git_file) | **POST** /environment/{environmentId}/checkGitFile | Check git file configuration is correct
[**check_helm_repository**](EnvironmentApi.md#check_helm_repository) | **POST** /environment/{environmentId}/checkHelmRepository | Check helm repository configuration is correct
[**list_environment_services_links**](EnvironmentApi.md#list_environment_services_links) | **GET** /environment/{environmentId}/link | List environment services links



## check_container_image

> serde_json::Value check_container_image(environment_id, container_image_check_request)
Check container image configuration is correct

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**container_image_check_request** | Option<[**ContainerImageCheckRequest**](ContainerImageCheckRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_dockerfile

> models::DockerfileCheckResponse check_dockerfile(environment_id, dockerfile_check_request)
Check dockerfile configuration is correct

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**dockerfile_check_request** | Option<[**DockerfileCheckRequest**](DockerfileCheckRequest.md)> |  |  |

### Return type

[**models::DockerfileCheckResponse**](DockerfileCheckResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_git_file

> serde_json::Value check_git_file(environment_id, git_file_check_request)
Check git file configuration is correct

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**git_file_check_request** | Option<[**GitFileCheckRequest**](GitFileCheckRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_helm_repository

> serde_json::Value check_helm_repository(environment_id, helm_check_request)
Check helm repository configuration is correct

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**helm_check_request** | Option<[**HelmCheckRequest**](HelmCheckRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_environment_services_links

> models::LinkResponseList list_environment_services_links(environment_id)
List environment services links

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |

### Return type

[**models::LinkResponseList**](LinkResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

