# DeployAllRequestHelmsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | id of the helm to be updated. | [optional]
**chart_version** | Option<**String**> | The new chart version for the Helm source. Use this only if the helm has a Helm repository source. | [optional]
**git_commit_id** | Option<**String**> | The commit Id to deploy. Use this only if the helm has a Git repository source. | [optional]
**values_override_git_commit_id** | Option<**String**> | The commit Id of the override values to deploy. Use only if the helm has a Git override values repository. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


