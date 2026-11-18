# ContainerGitRepositorySourceResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**has_access** | Option<**bool**> |  | [optional]
**provider** | [**models::GitProviderEnum**](GitProviderEnum.md) |  | 
**owner** | **String** |  | 
**url** | **String** |  | 
**name** | **String** | repository name | 
**branch** | Option<**String**> |  | [optional]
**root_path** | Option<**String**> |  | [optional]
**deployed_commit_id** | Option<**String**> | Git commit ID corresponding to the deployed version of the app | [optional]
**deployed_commit_date** | Option<**String**> | Git commit date corresponding to the deployed version of the app | [optional][readonly]
**deployed_commit_contributor** | Option<**String**> | Git commit user corresponding to the deployed version of the app | [optional]
**deployed_commit_tag** | Option<**String**> |  | [optional]
**git_token_id** | Option<**String**> |  | [optional]
**git_token_name** | Option<**String**> |  | [optional]
**dockerfile_path** | **String** | The path of the associated Dockerfile | 
**docker_target_build_stage** | Option<**String**> | The target build stage in the Dockerfile to build. If null, the last stage of the Dockerfile is built. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


