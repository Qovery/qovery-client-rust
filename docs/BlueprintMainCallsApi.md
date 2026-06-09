# \BlueprintMainCallsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_blueprint**](BlueprintMainCallsApi.md#create_blueprint) | **POST** /environment/{environmentId}/blueprint | Create a blueprint service in an environment
[**get_blueprint_catalog**](BlueprintMainCallsApi.md#get_blueprint_catalog) | **GET** /organization/{organizationId}/blueprint/catalog | Get the blueprint service catalog



## create_blueprint

> models::BlueprintResponse create_blueprint(environment_id, blueprint_create_request, deploy)
Create a blueprint service in an environment

Instantiates a blueprint from the service catalog into the given environment. Pass `deploy=true` to trigger an immediate deployment after creation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**blueprint_create_request** | [**BlueprintCreateRequest**](BlueprintCreateRequest.md) |  | [required] |
**deploy** | Option<**bool**> | Trigger a deployment immediately after creation |  |[default to false]

### Return type

[**models::BlueprintResponse**](BlueprintResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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

