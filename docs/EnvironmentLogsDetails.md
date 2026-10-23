# EnvironmentLogsDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | Option<**uuid::Uuid**> |  | [optional]
**cluster_id** | Option<**uuid::Uuid**> |  | [optional]
**execution_id** | Option<**String**> |  | [optional]
**transmitter** | Option<[**models::EnvironmentLogsDetailsTransmitter**](EnvironmentLogsDetailsTransmitter.md)> |  | [optional]
**stage** | Option<[**models::EnvironmentLogsDetailsStage**](EnvironmentLogsDetailsStage.md)> |  | [optional]
**step_name** | Option<**String**> | Step of the deployment this line belongs to: - pre-check: CLOUD_CREDENTIALS_CHECK, GIT_CHECK (the `name` of a `pre_check_stage` step); - stage queueing: QUEUEING; - build and deploy: REGISTRY_CREATE_REPOSITORY, GIT_CLONE, PROVISION_BUILDER, BUILD,   IMAGE_PUSH, CACHE_EXPORT, MIRROR_IMAGE, DEPLOYMENT, ROUTER_DEPLOYMENT or EXECUTING   (a step's `step_name`, or the `name` of a BUILD sub-step, in the deployment statuses); A line that belongs to no specific step (deployment start and end, environment-level messages) carries the step of the line before it from the same source (the environment, or the service), so lines grouped by step stay in time order. The phase (build or deploy) is `stage.step`. Absent on older deployments, on infrastructure (cluster) and blueprint logs, and on a line with no earlier step to follow: group those by `stage.step`. Clients must accept values not listed here.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


