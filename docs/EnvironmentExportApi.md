# \EnvironmentExportApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**export_environment_configuration_into_terraform**](EnvironmentExportApi.md#export_environment_configuration_into_terraform) | **GET** /environment/{environmentId}/terraformExport | Export full environment and its resources into Terraform manifests



## export_environment_configuration_into_terraform

> std::path::PathBuf export_environment_configuration_into_terraform(environment_id)
Export full environment and its resources into Terraform manifests

Secret values are never included in the export. Service secrets are preserved as empty input variables marked Redacted. Cloud, Git, registry and cluster credentials must be supplied through input variables without defaults.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

