# \ClusterDeploymentHistoryApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_cluster_deployment_history_v2**](ClusterDeploymentHistoryApi.md#list_cluster_deployment_history_v2) | **GET** /organization/{organizationId}/cluster/{clusterId}/deploymentHistoryV2 | List cluster deployments
[**list_cluster_deployment_logs**](ClusterDeploymentHistoryApi.md#list_cluster_deployment_logs) | **GET** /organization/{organizationId}/cluster/{clusterId}/deployment/{deploymentId}/logs | List logs for a specific cluster deployment



## list_cluster_deployment_history_v2

> models::ClusterDeploymentHistoryPaginatedResponseListV2 list_cluster_deployment_history_v2(organization_id, cluster_id, page_size)
List cluster deployments

List previous and current cluster deployments. It returns actual deployments only: dry-runs and stop/delete operations are excluded. By default it returns the 20 last results. Use the pageSize query parameter to adjust the number of returned results

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |
**page_size** | Option<**f64**> | The number of deployments to return in the current page. Must be greater than or equal to 1 |  |[default to 20]

### Return type

[**models::ClusterDeploymentHistoryPaginatedResponseListV2**](ClusterDeploymentHistoryPaginatedResponseListV2.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cluster_deployment_logs

> models::ClusterLogsResponseList list_cluster_deployment_logs(organization_id, cluster_id, deployment_id)
List logs for a specific cluster deployment

List the logs of a specific cluster deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |
**deployment_id** | **uuid::Uuid** | Deployment ID | [required] |

### Return type

[**models::ClusterLogsResponseList**](ClusterLogsResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

