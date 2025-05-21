# TerraformRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**description** | **String** |  | 
**auto_approve** | **bool** |  | 
**auto_deploy** | **bool** |  | 
**terraform_files_source** | [**models::TerraformRequestTerraformFilesSource**](TerraformRequest_terraform_files_source.md) |  | 
**terraform_variables_source** | [**models::TerraformVariablesSourceRequest**](TerraformVariablesSourceRequest.md) |  | 
**provider** | **String** |  | 
**provider_version** | [**models::TerraformProviderVersion**](TerraformProviderVersion.md) |  | 
**timeout_sec** | Option<**String**> |  | [optional]
**icon_uri** | Option<**String**> |  | [optional]
**job_resources** | [**models::TerraformRequestJobResources**](TerraformRequestJobResources.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


