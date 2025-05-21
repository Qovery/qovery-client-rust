# JobSourceDockerResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**git_repository** | Option<[**models::ApplicationGitRepository**](ApplicationGitRepository.md)> |  | [optional]
**dockerfile_path** | Option<**String**> | The path of the associated Dockerfile. Only if you are using build_mode = DOCKER | [optional]
**dockerfile_raw** | Option<**String**> | The content of your dockerfile if it is not stored inside your git repository | [optional]
**docker_target_build_stage** | Option<**String**> | The target build stage in the Dockerfile to build | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


