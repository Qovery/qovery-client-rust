# DeploymentHistoryServiceDetailsOneOf2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**image_name** | **String** |  | 
**tag** | **String** |  | 
**commit** | Option<[**models::Commit**](Commit.md)> |  | [optional]
**schedule** | Option<[**models::DeploymentHistoryServiceDetailsOneOf2Schedule**](DeploymentHistoryServiceDetailsOneOf2Schedule.md)> |  | [optional]
**job_type** | **JobType** |  (enum: CRON, LIFECYCLE) | 
**build_pod_name** | Option<**String**> | The build pod name prefix. Only set for jobs with a git source (Docker build). Null for container-source jobs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


