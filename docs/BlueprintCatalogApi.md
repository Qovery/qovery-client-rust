# \BlueprintCatalogApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_blueprint_catalog_service_manifest**](BlueprintCatalogApi.md#get_blueprint_catalog_service_manifest) | **GET** /organization/{organizationId}/blueprint/catalog/{provider}/{serviceFamily}/{serviceVersion}/manifest | Get the input fields to display for a blueprint catalog service
[**get_blueprint_catalog_service_readme**](BlueprintCatalogApi.md#get_blueprint_catalog_service_readme) | **GET** /organization/{organizationId}/blueprint/catalog/{provider}/{serviceFamily}/{serviceVersion}/readme | Get the README of a blueprint catalog service



## get_blueprint_catalog_service_manifest

> models::BlueprintManifestResponse get_blueprint_catalog_service_manifest(organization_id, provider, service_family, service_version, environment_id)
Get the input fields to display for a blueprint catalog service

Returns the list of form fields the console must display to deploy the selected blueprint, derived from the blueprint's qbm.yml manifest. Includes editable variables (overridable=true) and auto-sourced context variables (overridable=false, with a source).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**provider** | **String** | Cloud provider (e.g. aws, gcp, azure) | [required] |
**service_family** | **String** | Service family (e.g. mysql, postgresql) | [required] |
**service_version** | **String** | Service version (e.g. 8, 14) | [required] |
**environment_id** | **uuid::Uuid** | Environment ID used to resolve context variables | [required] |

### Return type

[**models::BlueprintManifestResponse**](BlueprintManifestResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blueprint_catalog_service_readme

> models::BlueprintReadmeResponse get_blueprint_catalog_service_readme(organization_id, provider, service_family, service_version)
Get the README of a blueprint catalog service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**provider** | **String** | Cloud provider (e.g. aws, gcp, azure) | [required] |
**service_family** | **String** | Service family (e.g. mysql, postgresql) | [required] |
**service_version** | **String** | Service version (e.g. 8, 14) | [required] |

### Return type

[**models::BlueprintReadmeResponse**](BlueprintReadmeResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

