# \JobBuildAnalysisApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_job_build_analysis**](JobBuildAnalysisApi.md#get_job_build_analysis) | **GET** /job/{jobId}/buildAnalysis | Get the build analysis of a job



## get_job_build_analysis

> models::BuildAnalysisState get_job_build_analysis(job_id)
Get the build analysis of a job

Returns the latest build of the service that can be analysed and its latest analysis, if any. Only one analysis is kept per service.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |

### Return type

[**models::BuildAnalysisState**](BuildAnalysisState.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

