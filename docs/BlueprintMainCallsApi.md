# \BlueprintMainCallsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_blueprint_catalog**](BlueprintMainCallsApi.md#get_blueprint_catalog) | **GET** /organization/{organizationId}/blueprint/catalog | Get the blueprint service catalog



## get_blueprint_catalog

> models::BlueprintCatalogResponse get_blueprint_catalog(organization_id)
Get the blueprint service catalog

Retrieves the Qovery service catalog from the public GitHub repository (Qovery/service-catalog). The catalog lists all available blueprints that can be deployed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::BlueprintCatalogResponse**](BlueprintCatalogResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

