# TerraformBackendBlueprint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Terraform backend type (e.g. s3, gcs, azurerm) | 
**config** | Option<**std::collections::HashMap<String, String>**> | Static backend configuration (bucket, region, etc.). Credentials should be provided via environment variables, not here. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


