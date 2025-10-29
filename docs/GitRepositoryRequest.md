# GitRepositoryRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** | Git repository URL | 
**branch** | Option<**String**> | Name of the branch to use (optional) | [optional]
**root_path** | Option<**String**> | Root path within the repository | [optional][default to /]
**git_token_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The git token id on Qovery side | [optional]
**provider** | [**models::GitProvider**](GitProvider.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


