# TerraformRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**description** | **String** |  | 
**auto_deploy** | **bool** |  | 
**terraform_files_source** | [**models::TerraformRequestTerraformFilesSource**](TerraformRequest_terraform_files_source.md) |  | 
**terraform_variables_source** | [**models::TerraformVariablesSourceRequest**](TerraformVariablesSourceRequest.md) |  | 
**backend** | [**models::TerraformBackend**](TerraformBackend.md) |  | 
**provider** | **String** |  | 
**provider_version** | [**models::TerraformProviderVersion**](TerraformProviderVersion.md) |  | 
**timeout_sec** | Option<**i32**> |  | [optional]
**icon_uri** | Option<**String**> |  | [optional]
**job_resources** | [**models::TerraformRequestJobResources**](TerraformRequestJobResources.md) |  | 
**use_cluster_credentials** | Option<**bool**> |  | [optional]
**action_extra_arguments** | Option<[**std::collections::HashMap<String, Vec<String>>**](Vec.md)> | The key represent the action command name i.e: \"plan\" The value represent the extra arguments to pass to this command  i.e: {\"apply\", [\"-lock=false\"]} is going to prepend `-lock=false` to terraform apply commands | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


