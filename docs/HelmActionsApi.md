# \HelmActionsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**deploy_helm**](HelmActionsApi.md#deploy_helm) | **POST** /helm/{helmId}/deploy | Deploy helm
[**redeploy_helm**](HelmActionsApi.md#redeploy_helm) | **POST** /helm/{helmId}/redeploy | Redeploy helm
[**stop_helm**](HelmActionsApi.md#stop_helm) | **POST** /helm/{helmId}/stop | Stop helm



## deploy_helm

> models::Status deploy_helm(helm_id, force_event, helm_deploy_request)
Deploy helm

You must provide a git commit id or a helm version depending on the source location of your code (git vs helm repository).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**helm_id** | **uuid::Uuid** | Helm ID | [required] |
**force_event** | Option<[**HelmForceEvent**](.md)> | When filled, it indicates the target event to be deployed.   If the concerned helm hasn't the target event provided, the helm won't be deployed.  |  |
**helm_deploy_request** | Option<[**HelmDeployRequest**](HelmDeployRequest.md)> |  |  |

### Return type

[**models::Status**](Status.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## redeploy_helm

> models::Status redeploy_helm(helm_id, force_event)
Redeploy helm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**helm_id** | **uuid::Uuid** | Helm ID | [required] |
**force_event** | Option<[**HelmForceEvent**](.md)> | When filled, it indicates the target event to be deployed.   If the concerned helm hasn't the target event provided, the helm won't be deployed.  |  |

### Return type

[**models::Status**](Status.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_helm

> models::Status stop_helm(helm_id)
Stop helm

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

