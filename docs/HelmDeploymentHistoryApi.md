# \HelmDeploymentHistoryApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_helm_deployment_history**](HelmDeploymentHistoryApi.md#list_helm_deployment_history) | **GET** /helm/{helmId}/deploymentHistory | List helm deployments
[**list_helm_deployment_history_v2**](HelmDeploymentHistoryApi.md#list_helm_deployment_history_v2) | **GET** /helm/{helmId}/deploymentHistoryV2 | List helm deployments



## list_helm_deployment_history

> models::ListHelmDeploymentHistory200Response list_helm_deployment_history(helm_id)
List helm deployments

Returns the 20 last helm deployments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**helm_id** | **uuid::Uuid** | Helm ID | [required] |

### Return type

[**models::ListHelmDeploymentHistory200Response**](listHelmDeploymentHistory_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_helm_deployment_history_v2

> models::DeploymentHistoryServicePaginatedResponseListV2 list_helm_deployment_history_v2(helm_id)
List helm deployments

Returns the 20 last helm deployments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**helm_id** | **uuid::Uuid** | Helm ID | [required] |

### Return type

[**models::DeploymentHistoryServicePaginatedResponseListV2**](DeploymentHistoryServicePaginatedResponseListV2.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

