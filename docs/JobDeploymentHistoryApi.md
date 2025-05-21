# \JobDeploymentHistoryApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_job_deployment_history**](JobDeploymentHistoryApi.md#list_job_deployment_history) | **GET** /job/{jobId}/deploymentHistory | List job deployments
[**list_job_deployment_history_v2**](JobDeploymentHistoryApi.md#list_job_deployment_history_v2) | **GET** /job/{jobId}/deploymentHistoryV2 | List job deployments



## list_job_deployment_history

> models::ListJobDeploymentHistory200Response list_job_deployment_history(job_id)
List job deployments

Returns the 20 last job deployments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |

### Return type

[**models::ListJobDeploymentHistory200Response**](listJobDeploymentHistory_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_job_deployment_history_v2

> models::DeploymentHistoryServicePaginatedResponseListV2 list_job_deployment_history_v2(job_id)
List job deployments

Returns the 20 last job deployments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |

### Return type

[**models::DeploymentHistoryServicePaginatedResponseListV2**](DeploymentHistoryServicePaginatedResponseListV2.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

