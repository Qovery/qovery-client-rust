# TerraformResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | **String** | name is case insensitive | 
**description** | Option<**String**> |  | [optional]
**timeout_sec** | **i32** |  | [default to 600]
**auto_approve** | **bool** |  | 
**auto_deploy** | **bool** |  | 
**terraform_files_source** | Option<[**models::TerraformFilesSource**](TerraformFilesSource.md)> |  | [optional]
**icon_uri** | **String** | Icon URI representing the terraform service. | 
**service_type** | [**models::ServiceTypeEnum**](ServiceTypeEnum.md) |  | 
**terraform_variables_source** | [**models::TerraformVariablesSourceResponse**](TerraformVariablesSourceResponse.md) |  | 
**provider** | **String** |  | 
**backend** | [**models::TerraformBackend**](TerraformBackend.md) |  | 
**provider_version** | [**models::TerraformProviderVersion**](TerraformProviderVersion.md) |  | 
**job_resources** | [**models::TerraformJobResourcesResponse**](TerraformJobResourcesResponse.md) |  | 
**environment** | [**models::ReferenceObject**](ReferenceObject.md) |  | 
**use_cluster_credentials** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


