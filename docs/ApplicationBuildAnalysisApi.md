# \ApplicationBuildAnalysisApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_application_build_analysis**](ApplicationBuildAnalysisApi.md#get_application_build_analysis) | **GET** /application/{applicationId}/buildAnalysis | Get the build analysis of an application



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

