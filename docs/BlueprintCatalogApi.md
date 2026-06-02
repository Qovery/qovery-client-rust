# \BlueprintCatalogApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_blueprint_catalog_service_readme**](BlueprintCatalogApi.md#get_blueprint_catalog_service_readme) | **GET** /organization/{organizationId}/blueprint/catalog/{provider}/{serviceFamily}/{serviceVersion}/readme | Get the README of a blueprint catalog service



## get_blueprint_catalog_service_readme

> String get_blueprint_catalog_service_readme(organization_id, provider, service_family, service_version)
Get the README of a blueprint catalog service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**provider** | **String** | Cloud provider (e.g. aws, gcp, azure) | [required] |
**service_family** | **String** | Service family (e.g. mysql, postgresql) | [required] |
**service_version** | **String** | Service version (e.g. 8, 14) | [required] |

### Return type

**String**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/markdown, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

