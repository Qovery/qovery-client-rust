# \AdminApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_public_service_version**](AdminApi.md#get_public_service_version) | **GET** /engine/serviceVersion | Get a service version
[**list_user_sign_ups**](AdminApi.md#list_user_sign_ups) | **GET** /admin/listUserSignUp | Search user signups
[**store_cli_demo_debug_logs**](AdminApi.md#store_cli_demo_debug_logs) | **POST** /admin/demoDebugLog | Store CLI demo debug logs



## get_public_service_version

> models::EngineVersionResponse get_public_service_version(service_type)
Get a service version

Get the version of an engine related service. Worker service types are unavailable through this route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_type** | **String** |  | [required] |

### Return type

[**models::EngineVersionResponse**](EngineVersionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_sign_ups

> models::UserSignUpResponseList list_user_sign_ups(search)
Search user signups

Search user signups as a Qovery administrator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | **String** |  | [required] |

### Return type

[**models::UserSignUpResponseList**](UserSignUpResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_cli_demo_debug_logs

> store_cli_demo_debug_logs(organization, cluster_name, body)
Store CLI demo debug logs

Store CLI demo debug logs for an organization and cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **uuid::Uuid** |  | [required] |
**cluster_name** | **String** |  | [required] |
**body** | Option<**std::path::PathBuf**> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

