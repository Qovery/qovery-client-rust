# \UserSignUpApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user_sign_up**](UserSignUpApi.md#create_user_sign_up) | **POST** /admin/userSignUp | Send Sign Up request
[**get_user_sign_up**](UserSignUpApi.md#get_user_sign_up) | **GET** /admin/userSignUp | Get Sign up information



## create_user_sign_up

> create_user_sign_up(sign_up_request)
Send Sign Up request

Send a Sign Up request containing the user information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sign_up_request** | Option<[**SignUpRequest**](SignUpRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_sign_up

> models::SignUp get_user_sign_up()
Get Sign up information

Retrieve the Sign Up information of the user

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SignUp**](SignUp.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

