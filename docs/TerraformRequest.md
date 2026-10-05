# TerraformRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional][default to ]
**auto_deploy_config** | Option<[**models::TerraformAutoDeployConfig**](TerraformAutoDeployConfig.md)> |  | [optional]
**auto_deploy** | Option<**bool**> | Legacy alternative to auto_deploy_config. | [optional]
**auto_preview** | Option<**bool**> |  | [optional]
**terraform_files_source** | [**models::TerraformRequestTerraformFilesSource**](TerraformRequestTerraformFilesSource.md) |  | 
**terraform_variables_source** | [**models::TerraformVariablesSourceRequest**](TerraformVariablesSourceRequest.md) |  | 
**backend** | Option<[**models::TerraformBackend**](TerraformBackend.md)> |  | [optional]
**engine** | Option<[**models::TerraformEngineEnum**](TerraformEngineEnum.md)> |  | [optional]
**provider_version** | [**models::TerraformProviderVersion**](TerraformProviderVersion.md) |  | 
**timeout_sec** | Option<**i32**> |  | [optional]
**icon_uri** | Option<**String**> |  | [optional]
**job_resources** | [**models::TerraformRequestJobResources**](TerraformRequestJobResources.md) |  | 
**use_cluster_credentials** | Option<**bool**> |  | [optional]
**action_extra_arguments** | Option<[**std::collections::HashMap<String, Vec<String>>**](Vec.md)> | The key represent the action command name i.e: \"plan\" The value represent the extra arguments to pass to this command  i.e: {\"apply\", [\"-lock=false\"]} is going to prepend `-lock=false` to terraform apply commands | [optional]
**dockerfile_fragment** | Option<[**models::TerraformRequestDockerfileFragment**](TerraformRequestDockerfileFragment.md)> |  | [optional]
**blueprint_id** | Option<**uuid::Uuid**> | The blueprint ID the service has been created from  | [optional]
**build_settings** | Option<[**models::BuildSettings**](BuildSettings.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


