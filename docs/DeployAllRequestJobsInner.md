# DeployAllRequestJobsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | id of the job to be updated. | [optional]
**image_tag** | Option<**String**> | new tag for the job image. Use only if job is an image source. Can be empty only if the service has been already deployed (in this case the service version won't be changed) | [optional]
**git_commit_id** | Option<**String**> | Commit ID to deploy. Use only if job is a repository source. Can be empty only if the service has been already deployed (in this case the service version won't be changed) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


