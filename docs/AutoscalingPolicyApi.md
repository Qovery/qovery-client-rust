# \AutoscalingPolicyApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_or_update_service_autoscaling**](AutoscalingPolicyApi.md#create_or_update_service_autoscaling) | **PUT** /service/{serviceId}/autoscalingPolicy | createOrUpdateServiceAutoscaling
[**delete_service_autoscaling**](AutoscalingPolicyApi.md#delete_service_autoscaling) | **DELETE** /service/{serviceId}/autoscalingPolicy | deleteServiceAutoscaling
[**get_service_autoscaling**](AutoscalingPolicyApi.md#get_service_autoscaling) | **GET** /service/{serviceId}/autoscalingPolicy | getServiceAutoscaling



## create_or_update_service_autoscaling

> models::KedaAutoscalingResponse create_or_update_service_autoscaling(service_id, body)
createOrUpdateServiceAutoscaling

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **uuid::Uuid** | Service ID of an application/job/container/database | [required] |
**body** | Option<**models::KedaAutoscalingRequest**> |  |  |

### Return type

[**models::KedaAutoscalingResponse**](KedaAutoscalingResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_service_autoscaling

> delete_service_autoscaling(service_id)
deleteServiceAutoscaling

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **uuid::Uuid** | Service ID of an application/job/container/database | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_autoscaling

> models::KedaAutoscalingResponse get_service_autoscaling(service_id)
getServiceAutoscaling

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **uuid::Uuid** | Service ID of an application/job/container/database | [required] |

### Return type

[**models::KedaAutoscalingResponse**](KedaAutoscalingResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

