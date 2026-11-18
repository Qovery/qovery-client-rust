# ContainerDeployRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**uuid::Uuid**> |  | [optional]
**image_tag** | Option<**String**> | Image tag to deploy, for a container with an image source. Sending it for a container with a Git repository source returns 400. A `null` value counts as not sent.  | [optional]
**git_commit_id** | Option<**String**> | Commit to deploy, for a container with a Git repository source. When not specified, the container's stored commit is deployed. Sending it for a container with an image source returns 400. A `null` value counts as not sent.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


