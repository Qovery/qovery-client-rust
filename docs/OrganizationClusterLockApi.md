# \OrganizationClusterLockApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_cluster_lock**](OrganizationClusterLockApi.md#list_cluster_lock) | **GET** /organization/{organizationId}/lock | List locked Cluster by organization



## list_cluster_lock

> models::ClusterLockList list_cluster_lock(organization_id)
List locked Cluster by organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |

### Return type

[**models::ClusterLockList**](ClusterLockList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

