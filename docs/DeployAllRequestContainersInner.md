# DeployAllRequestContainersInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | id of the container to be updated. | 
**image_tag** | Option<**String**> | new tag for the container. Can be empty only if the service has been already deployed (in this case the service version won't be changed)  For a container with a Git repository source: - an absent or empty `image_tag`, or one equal to the `tag` that `GET /container/{containerId}` returns, deploys the container at `git_commit_id`, or at its stored commit when `git_commit_id` is absent or empty; - a different `image_tag` returns 400.  | [optional]
**git_commit_id** | Option<**String**> | Commit to deploy, for a container with a Git repository source. When absent or empty, the container's stored commit is deployed. On a container with a Git repository source, a value that is not a 40-character SHA-1 commit id returns 400. Ignored for a container with an image source.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


