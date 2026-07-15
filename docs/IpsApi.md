# \IpsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_qovery_ips**](IpsApi.md#list_qovery_ips) | **GET** /ips | List Qovery NAT gateway IP addresses



## list_qovery_ips

> models::QoveryIpsResponse list_qovery_ips()
List Qovery NAT gateway IP addresses

Returns the list of static IP addresses used by Qovery's NAT gateways for outbound traffic. Customers can allow-list these IPs on their side. This endpoint is public and does not require authentication.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::QoveryIpsResponse**](QoveryIpsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

