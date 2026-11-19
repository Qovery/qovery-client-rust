# \ContainerMainCallsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_container**](ContainerMainCallsApi.md#delete_container) | **DELETE** /container/{containerId} | Delete container
[**edit_container**](ContainerMainCallsApi.md#edit_container) | **PUT** /container/{containerId} | Edit container
[**get_container**](ContainerMainCallsApi.md#get_container) | **GET** /container/{containerId} | Get container by ID
[**get_container_status**](ContainerMainCallsApi.md#get_container_status) | **GET** /container/{containerId}/status | Get container status
[**list_container_links**](ContainerMainCallsApi.md#list_container_links) | **GET** /container/{containerId}/link | List all URLs of the container



## delete_container

> delete_container(container_id, skip_reconcile)
Delete container

To delete the container you must have the admin permission

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** | Container ID | [required] |
**skip_reconcile** | Option<**bool**> | When true, skip the pre-destroy apply/reconcile and tear down best-effort, tolerating already-absent resources. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_container

> models::ContainerResponse edit_container(container_id, container_request)
Edit container

- To edit the container you must have the admin permission. - For port edition, if you provide a port id, we will update the corresponding port. If you don't we will create a new one. If you remove a port from the payload, we will delete it. - For storage edition, if you provide a storage id, we will update the corresponding storage. If you don't we will create a new one. If you remove a storage from the payload, we will delete it. - To switch the container between an image and a Git repository, send `source`. The container keeps its id, Kubernetes names, volumes and built-in variables. When `source` is not sent, the stored source is kept. - A body built from the `GET /container/{containerId}` response is accepted as is: `source` is ignored when it repeats the stored source, and `registry_id`, `image_name` and `tag` when they repeat the image the container runs. Only what changed is applied. To change the image with the flat fields, send all three of `registry_id`, `image_name` and `tag`: GET returns `registry.id`, not `registry_id`.  Returns 400 when: - `source` and `registry_id`, `image_name` or `tag` both ask for a change, unless both name the same image; - `source` has both `image` and `git_repository`, or neither; - the container has an image source, only some of `registry_id`, `image_name` and `tag` are sent, and either `source` is not sent or one of them differs from the image the container runs: send all three, or none to keep the image; - the container has a Git repository source, `source.image` is not sent, and `registry_id`, `image_name` or `tag` differ from the `registry.id`, `image_name` and `tag` that `GET /container/{containerId}` returns. A deployment that finishes between the GET and the PUT changes these values, so the PUT gets a 400: get the container again and retry; - `auto_deploy` or `auto_preview` is `true` while `source.git_repository` is sent, or while the container has a Git repository source and `source` is not sent; - `build_settings` is sent while the container keeps, or switches to, an image source. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** | Container ID | [required] |
**container_request** | Option<[**ContainerRequest**](ContainerRequest.md)> |  |  |

### Return type

[**models::ContainerResponse**](ContainerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_container

> models::ContainerResponse get_container(container_id)
Get container by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** | Container ID | [required] |

### Return type

[**models::ContainerResponse**](ContainerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_container_status

> models::Status get_container_status(container_id)
Get container status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** | Container ID | [required] |

### Return type

[**models::Status**](Status.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_container_links

> models::LinkResponseList list_container_links(container_id)
List all URLs of the container

This will return all the custom domains and Qovery autogenerated domain for the given container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** | Container ID | [required] |

### Return type

[**models::LinkResponseList**](LinkResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

