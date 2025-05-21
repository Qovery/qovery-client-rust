# \JobMainCallsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_job**](JobMainCallsApi.md#delete_job) | **DELETE** /job/{jobId} | Delete job
[**edit_job**](JobMainCallsApi.md#edit_job) | **PUT** /job/{jobId} | Edit job
[**get_job**](JobMainCallsApi.md#get_job) | **GET** /job/{jobId} | Get job by ID
[**get_job_status**](JobMainCallsApi.md#get_job_status) | **GET** /job/{jobId}/status | Get job status
[**list_job_commit**](JobMainCallsApi.md#list_job_commit) | **GET** /job/{jobId}/commit | List last job commits



## delete_job

> delete_job(job_id)
Delete job

To delete the job you must have the admin permission

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_job

> models::JobResponse edit_job(job_id, job_request)
Edit job

- To edit the job you must have the admin permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |
**job_request** | Option<[**JobRequest**](JobRequest.md)> |  |  |

### Return type

[**models::JobResponse**](JobResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job

> models::JobResponse get_job(job_id)
Get job by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |

### Return type

[**models::JobResponse**](JobResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_status

> models::Status get_job_status(job_id)
Get job status

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


## list_job_commit

> models::CommitResponseList list_job_commit(job_id, start_id, git_commit_id)
List last job commits

Returns list of the last 100 commits made on the repository linked to the job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |
**start_id** | Option<**uuid::Uuid**> | Starting point after which to return results |  |
**git_commit_id** | Option<**uuid::Uuid**> | Git Commit ID |  |

### Return type

[**models::CommitResponseList**](CommitResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

