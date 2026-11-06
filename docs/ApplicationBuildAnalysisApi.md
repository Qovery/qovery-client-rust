# \ApplicationBuildAnalysisApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_application_build_analysis**](ApplicationBuildAnalysisApi.md#get_application_build_analysis) | **GET** /application/{applicationId}/buildAnalysis | Get the build analysis of an application
[**start_application_build_analysis**](ApplicationBuildAnalysisApi.md#start_application_build_analysis) | **POST** /application/{applicationId}/buildAnalysis | Analyse the latest build of an application



## get_application_build_analysis

> models::BuildAnalysisState get_application_build_analysis(application_id)
Get the build analysis of an application

Returns the latest build of the service that can be analysed and its latest analysis, if any. Only one analysis is kept per service.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |

### Return type

[**models::BuildAnalysisState**](BuildAnalysisState.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_application_build_analysis

> models::BuildAnalysisResponse start_application_build_analysis(application_id, build_analysis_start_request)
Analyse the latest build of an application

Starts an AI analysis of the latest build of the service. It runs in the background and reports its progress on the websocket route `/build/analysis`. The result gives a confidence, the estimated time saved, what was reviewed, a summary, the improvements to make, a prompt for the customer's own AI agent, and a reworked Dockerfile when an improvement is a Dockerfile change. Starting an analysis replaces the previous one. It never changes the build or the image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |
**build_analysis_start_request** | [**BuildAnalysisStartRequest**](BuildAnalysisStartRequest.md) |  | [required] |

### Return type

[**models::BuildAnalysisResponse**](BuildAnalysisResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

