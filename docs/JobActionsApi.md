# \JobActionsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clean_failed_job**](JobActionsApi.md#clean_failed_job) | **POST** /job/{jobId}/cleanFailedJob | Clean a failed job
[**deploy_job**](JobActionsApi.md#deploy_job) | **POST** /job/{jobId}/deploy | Deploy job
[**redeploy_job**](JobActionsApi.md#redeploy_job) | **POST** /job/{jobId}/redeploy | Redeploy job
[**stop_job**](JobActionsApi.md#stop_job) | **POST** /job/{jobId}/stop | Stop job
[**uninstall_job**](JobActionsApi.md#uninstall_job) | **POST** /job/{jobId}/uninstall | Uninstall job



## clean_failed_job

> models::CleanFailedJob200Response clean_failed_job(job_id)
Clean a failed job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |

### Return type

[**models::CleanFailedJob200Response**](cleanFailedJob_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deploy_job

> models::Status deploy_job(job_id, force_event, job_deploy_request)
Deploy job

You must provide a git commit id or an image tag depending on the source location of your code (git vs image repository).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |
**force_event** | Option<[**JobForceEvent**](.md)> | When filled, it indicates the target event to be deployed.   If the concerned job hasn't the target event provided, the job won't be deployed.  |  |
**job_deploy_request** | Option<[**JobDeployRequest**](JobDeployRequest.md)> |  |  |

### Return type

[**models::Status**](Status.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## redeploy_job

> models::Status redeploy_job(job_id, force_event)
Redeploy job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |
**force_event** | Option<[**JobForceEvent**](.md)> | When filled, it indicates the target event to be deployed.   If the concerned job hasn't the target event provided, the job won't be deployed.  |  |

### Return type

[**models::Status**](Status.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_job

> models::Status stop_job(job_id)
Stop job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |

### Return type

[**models::Status**](Status.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uninstall_job

> serde_json::Value uninstall_job(job_id, body)
Uninstall job

Delete the resources of the job but keep Qovery configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

