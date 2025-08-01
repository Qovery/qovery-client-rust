# ApplicationGitRepositoryRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** | application git repository URL | 
**branch** | Option<**String**> | Name of the branch to use. This is optional If not specified, then the branch used is the `main` or `master` one  | [optional]
**root_path** | Option<**String**> | indicates the root path of the application. | [optional][default to /]
**git_token_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The git token id on Qovery side | [optional]
**provider** | [**models::GitProviderEnum**](GitProviderEnum.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


