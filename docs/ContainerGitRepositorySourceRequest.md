# ContainerGitRepositorySourceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** | Git repository URL | 
**branch** | Option<**String**> | Name of the branch to use. This is optional If not specified, then the branch used is the `main` or `master` one  | [optional]
**root_path** | Option<**String**> | indicates the root path in the repository. | [optional][default to /]
**git_token_id** | Option<**uuid::Uuid**> | The git token id on Qovery side | [optional]
**provider** | [**models::GitProviderEnum**](GitProviderEnum.md) |  | 
**dockerfile_path** | **String** | The path of the associated Dockerfile | 
**docker_target_build_stage** | Option<**String**> | The target build stage in the Dockerfile to build. If null, the last stage of the Dockerfile is built. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


