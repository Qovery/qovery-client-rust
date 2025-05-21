# \HelmMainCallsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_helm**](HelmMainCallsApi.md#delete_helm) | **DELETE** /helm/{helmId} | Delete helm
[**edit_helm**](HelmMainCallsApi.md#edit_helm) | **PUT** /helm/{helmId} | Edit helm
[**get_helm**](HelmMainCallsApi.md#get_helm) | **GET** /helm/{helmId} | Get helm by ID
[**get_helm_kubernetes_services**](HelmMainCallsApi.md#get_helm_kubernetes_services) | **GET** /helm/{helmId}/listServices | Get helm kubernetes services
[**get_helm_status**](HelmMainCallsApi.md#get_helm_status) | **GET** /helm/{helmId}/status | Get helm status
[**list_helm_commit**](HelmMainCallsApi.md#list_helm_commit) | **GET** /helm/{helmId}/commit | List last helm commits
[**list_helm_links**](HelmMainCallsApi.md#list_helm_links) | **GET** /helm/{helmId}/link | List all URLs of the helm



## delete_helm

> delete_helm(helm_id)
Delete helm

To delete the helm you must have the admin permission

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**helm_id** | **uuid::Uuid** | Helm ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_helm

> models::HelmResponse edit_helm(helm_id, helm_request)
Edit helm

- To edit the helm you must have the admin permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**helm_id** | **uuid::Uuid** | Helm ID | [required] |
**helm_request** | Option<[**HelmRequest**](HelmRequest.md)> |  |  |

### Return type

[**models::HelmResponse**](HelmResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_helm

> models::HelmResponse get_helm(helm_id)
Get helm by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**helm_id** | **uuid::Uuid** | Helm ID | [required] |

### Return type

[**models::HelmResponse**](HelmResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_helm_kubernetes_services

> models::KubernetesServiceResponseList get_helm_kubernetes_services(helm_id)
Get helm kubernetes services

Get helm kubernetes services

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**helm_id** | **String** |  | [required] |

### Return type

[**models::KubernetesServiceResponseList**](KubernetesServiceResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_helm_status

> models::Status get_helm_status(helm_id)
Get helm status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**helm_id** | **uuid::Uuid** | Helm ID | [required] |

### Return type

[**models::Status**](Status.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_helm_commit

> models::CommitResponseList list_helm_commit(helm_id, of)
List last helm commits

Returns list of the last 100 commits made on the repository linked to helm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**helm_id** | **uuid::Uuid** | Helm ID | [required] |
**of** | Option<**String**> | Source of git commit. Can be 'chart' or 'values' |  |[default to chart]

### Return type

[**models::CommitResponseList**](CommitResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_helm_links

> models::LinkResponseList list_helm_links(helm_id)
List all URLs of the helm

This will return all the custom domains and Qovery autogenerated domain for the given helm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**helm_id** | **uuid::Uuid** | Helm ID | [required] |

### Return type

[**models::LinkResponseList**](LinkResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

