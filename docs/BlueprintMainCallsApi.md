# \BlueprintMainCallsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_blueprint_update**](BlueprintMainCallsApi.md#check_blueprint_update) | **GET** /blueprint/{blueprintId}/update | Check if a blueprint service has an available update
[**create_blueprint**](BlueprintMainCallsApi.md#create_blueprint) | **POST** /environment/{environmentId}/blueprint | Create a blueprint service in an environment
[**get_blueprint_catalog**](BlueprintMainCallsApi.md#get_blueprint_catalog) | **GET** /organization/{organizationId}/blueprint/catalog | Get the blueprint service catalog
[**preview_blueprint_update**](BlueprintMainCallsApi.md#preview_blueprint_update) | **POST** /blueprint/{blueprintId}/update/preview | Preview a blueprint update
[**update_blueprint**](BlueprintMainCallsApi.md#update_blueprint) | **PATCH** /blueprint/{blueprintId} | Update a blueprint service



## check_blueprint_update

> models::BlueprintUpdateResponse check_blueprint_update(blueprint_id)
Check if a blueprint service has an available update

Returns the update availability for a deployed blueprint service, including the latest tag, and a diff of variables that are new, changed, or removed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blueprint_id** | **uuid::Uuid** | Blueprint ID | [required] |

### Return type

[**models::BlueprintUpdateResponse**](BlueprintUpdateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## preview_blueprint_update

> models::BlueprintUpdatePreviewResponse preview_blueprint_update(blueprint_id, blueprint_update_preview_request)
Preview a blueprint update

Dry-runs a blueprint update without persisting any changes. Returns a preview ID and the resolved service type. Both `variables` and `spec_overrides` follow RFC 7396 patch semantics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blueprint_id** | **uuid::Uuid** | Blueprint ID | [required] |
**blueprint_update_preview_request** | [**BlueprintUpdatePreviewRequest**](BlueprintUpdatePreviewRequest.md) |  | [required] |

### Return type

[**models::BlueprintUpdatePreviewResponse**](BlueprintUpdatePreviewResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_blueprint

> models::BlueprintResponse update_blueprint(blueprint_id, blueprint_update_request)
Update a blueprint service

Persists new values for a deployed blueprint service. Intended to be called after reviewing the diff returned by GET /blueprint/{blueprintId}/update. `variables` and `spec_overrides` follow JSON Merge Patch (RFC 7396) semantics: non-null value on a key upserts it, null value removes it, absent keys are left untouched, and passing null for the whole field leaves all existing values unchanged. **Note:** `name`, `tag`, and `icon` are required on every call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blueprint_id** | **uuid::Uuid** | Blueprint ID | [required] |
**blueprint_update_request** | [**BlueprintUpdateRequest**](BlueprintUpdateRequest.md) |  | [required] |

### Return type

[**models::BlueprintResponse**](BlueprintResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

