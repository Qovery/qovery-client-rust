# \ServiceStatusApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_ingress_deployment_status**](ServiceStatusApi.md#get_ingress_deployment_status) | **GET** /{serviceType}/{serviceId}/ingressDeploymentStatus | Get Ingress Deployment Status By Service



## get_ingress_deployment_status

> models::IngressDeploymentStatusResponse get_ingress_deployment_status(service_type, service_id)
Get Ingress Deployment Status By Service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_type** | **String** |  | [required] |
**service_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::IngressDeploymentStatusResponse**](IngressDeploymentStatusResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

