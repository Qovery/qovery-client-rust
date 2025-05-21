# HelmRepositoryRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**kind** | [**models::HelmRepositoryKindEnum**](HelmRepositoryKindEnum.md) |  | 
**description** | Option<**String**> |  | [optional]
**url** | Option<**String**> | URL of the helm chart repository: * For `OCI`: it must start by oci:// * For `HTTPS`: it must be start by https://  | [optional]
**skip_tls_verification** | **bool** | Bypass tls certificate verification when connecting to repository | 
**config** | [**models::HelmRepositoryRequestConfig**](HelmRepositoryRequest_config.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


