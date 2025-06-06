# \DefaultApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_cluster_token_by_cluster_id**](DefaultApi.md#get_cluster_token_by_cluster_id) | **GET** /cluster/{clusterId}/token | Get cluster token by clusterId
[**get_deployment_status_by_deployment_request_id**](DefaultApi.md#get_deployment_status_by_deployment_request_id) | **GET** /environment/deploymentStatus | Get Deployment Status By DeploymentRequestId



## get_cluster_token_by_cluster_id

> models::GetClusterTokenByClusterId200Response get_cluster_token_by_cluster_id(cluster_id)
Get cluster token by clusterId

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **String** |  | [required] |

### Return type

[**models::GetClusterTokenByClusterId200Response**](get_cluster_token_by_clusterId_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deployment_status_by_deployment_request_id

> models::EnvDeploymentStatus get_deployment_status_by_deployment_request_id(deployment_request_id)
Get Deployment Status By DeploymentRequestId

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_request_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::EnvDeploymentStatus**](EnvDeploymentStatus.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

