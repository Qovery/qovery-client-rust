# \AccountInfoApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**edit_account_information**](AccountInfoApi.md#edit_account_information) | **PUT** /account | Edit account information
[**get_account_information**](AccountInfoApi.md#get_account_information) | **GET** /account | Get Account information



## edit_account_information

> models::AccountInfo edit_account_information(account_info_edit_request)
Edit account information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_info_edit_request** | Option<[**AccountInfoEditRequest**](AccountInfoEditRequest.md)> |  |  |

### Return type

[**models::AccountInfo**](AccountInfo.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_information

> models::AccountInfo get_account_information()
Get Account information

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AccountInfo**](AccountInfo.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

