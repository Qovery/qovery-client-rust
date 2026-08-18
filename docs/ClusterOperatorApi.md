# \ClusterOperatorApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**attach_cluster_operator**](ClusterOperatorApi.md#attach_cluster_operator) | **POST** /organization/{organizationId}/cluster/{clusterId}/operator/attach | Attach a cluster to the Qovery Operator execution path
[**get_cluster_operator_bootstrap**](ClusterOperatorApi.md#get_cluster_operator_bootstrap) | **GET** /organization/{organizationId}/cluster/{clusterId}/operator/bootstrap | Get the Qovery Operator bootstrap
[**get_cluster_operator_status**](ClusterOperatorApi.md#get_cluster_operator_status) | **GET** /organization/{organizationId}/cluster/{clusterId}/operator/status | Get the Qovery Operator status for a cluster
[**list_cluster_operator_fleet**](ClusterOperatorApi.md#list_cluster_operator_fleet) | **GET** /admin/operator/clusters | List the Qovery Operator fleet
[**update_cluster_operator**](ClusterOperatorApi.md#update_cluster_operator) | **POST** /organization/{organizationId}/cluster/{clusterId}/operator/update | Update the Qovery Operator on a cluster



## attach_cluster_operator

> attach_cluster_operator(organization_id, cluster_id)
Attach a cluster to the Qovery Operator execution path

Marks a self-managed cluster for routing through the Qovery Operator. This records the routing intent only; it does not install the Operator or prove that it is connected. Cluster administrator permission is required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_operator_bootstrap

> models::ClusterOperatorBootstrapResponse get_cluster_operator_bootstrap(organization_id, cluster_id)
Get the Qovery Operator bootstrap

Compiles the Helm chart coordinates, values file, and ready-to-run Helm command for a self-managed cluster. The response contains cluster credentials and must be handled as sensitive data. Cluster administrator permission is required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |

### Return type

[**models::ClusterOperatorBootstrapResponse**](ClusterOperatorBootstrapResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_operator_status

> models::ClusterOperatorStatusResponse get_cluster_operator_status(organization_id, cluster_id)
Get the Qovery Operator status for a cluster

Returns heartbeat freshness and the image, chart, and protocol versions last reported by the running Operator, together with the current bootstrap targets and a stable drift status. A false operator_connected value means that no heartbeat was received within the presence window. The status is a display-version drift verdict, not an immutable release compatibility decision. Cluster viewer permission is sufficient.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |

### Return type

[**models::ClusterOperatorStatusResponse**](ClusterOperatorStatusResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cluster_operator_fleet

> models::ClusterOperatorFleetInventoryResponseList list_cluster_operator_fleet()
List the Qovery Operator fleet

Returns every self-managed cluster with its attachment, heartbeat freshness, desired and reported image and Helm chart versions, and a stable drift status. This operation is restricted to Qovery administrators and is the source for internal fleet CLI and dashboard consumers.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ClusterOperatorFleetInventoryResponseList**](ClusterOperatorFleetInventoryResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_cluster_operator

> models::ClusterOperatorUpdateResponse update_cluster_operator(organization_id, cluster_id, cluster_operator_update_request)
Update the Qovery Operator on a cluster

Queues an Engine v2 execution containing only the Qovery Operator Helm release. The chart version is required; the optional image version overrides the image target selected by q-core. The current Operator must be attached, connected, and protocol-compatible. A successful response means that the execution was accepted, not that Helm has completed. Cluster administrator permission is required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |
**cluster_operator_update_request** | [**ClusterOperatorUpdateRequest**](ClusterOperatorUpdateRequest.md) |  | [required] |

### Return type

[**models::ClusterOperatorUpdateResponse**](ClusterOperatorUpdateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

