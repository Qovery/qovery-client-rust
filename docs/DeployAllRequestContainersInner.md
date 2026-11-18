# DeployAllRequestContainersInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | id of the container to be updated. | 
**image_tag** | Option<**String**> | new tag for the container. Can be empty only if the service has been already deployed (in this case the service version won't be changed)  For a container with a Git repository source: - an absent or empty `image_tag`, or one equal to the `tag` that `GET /container/{containerId}` returns, deploys the container at its stored commit; - a different `image_tag` returns 400.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


