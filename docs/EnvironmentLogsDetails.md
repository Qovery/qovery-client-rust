# EnvironmentLogsDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | Option<**uuid::Uuid**> |  | [optional]
**cluster_id** | Option<**uuid::Uuid**> |  | [optional]
**execution_id** | Option<**String**> |  | [optional]
**transmitter** | Option<[**models::EnvironmentLogsDetailsTransmitter**](EnvironmentLogsDetailsTransmitter.md)> |  | [optional]
**stage** | Option<[**models::EnvironmentLogsDetailsStage**](EnvironmentLogsDetailsStage.md)> |  | [optional]
**step_name** | Option<**String**> | Step of the deployment this line belongs to: REGISTRY_CREATE_REPOSITORY, GIT_CLONE, PROVISION_BUILDER, BUILD, IMAGE_PUSH, CACHE_EXPORT, MIRROR_IMAGE, DEPLOYMENT, ROUTER_DEPLOYMENT or EXECUTING. It matches a step's `step_name`, or the `name` of a BUILD sub-step, in the deployment statuses. The phase (build or deploy) is `stage.step`. Absent on older deployments and on pause, delete and restart. Clients must accept values not listed here.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


