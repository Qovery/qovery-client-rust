# BlueprintManifestEngineTerraform

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** |  (enum: terraform) | 
**provider** | [**models::CloudVendorEnum**](CloudVendorEnum.md) |  | 
**terraform** | [**models::BlueprintManifestVersionBlock**](BlueprintManifestVersionBlock.md) |  | 
**credentials** | Option<[**models::BlueprintManifestCredentialsConfig**](BlueprintManifestCredentialsConfig.md)> |  | [optional]
**backend** | Option<[**models::BlueprintManifestBackendConfig**](BlueprintManifestBackendConfig.md)> |  | [optional]
**timeout** | Option<**i64**> | Default execution timeout in seconds. | [optional]
**resources** | Option<[**models::BlueprintManifestResourcesConfig**](BlueprintManifestResourcesConfig.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


