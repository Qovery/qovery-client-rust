# ApplicationEditRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**storage** | Option<[**Vec<models::ServiceStorageRequestStorageInner>**](ServiceStorageRequest_storage_inner.md)> |  | [optional]
**name** | Option<**String**> | name is case insensitive | [optional]
**description** | Option<**String**> | give a description to this application | [optional]
**git_repository** | Option<[**models::ApplicationGitRepositoryRequest**](ApplicationGitRepositoryRequest.md)> |  | [optional]
**build_mode** | Option<[**models::BuildModeEnum**](BuildModeEnum.md)> |  | [optional]
**dockerfile_path** | Option<**String**> | The path of the associated Dockerfile | [optional]
**cpu** | Option<**i32**> | unit is millicores (m). 1000m = 1 cpu | [optional][default to 500]
**memory** | Option<**i32**> | unit is MB. 1024 MB = 1GB | [optional][default to 512]
**gpu** | Option<**i32**> |  | [optional][default to 0]
**min_running_instances** | Option<**i32**> | Minimum number of instances running. This resource auto-scale based on the CPU and Memory consumption. Note: 0 means that there is no application running.  | [optional][default to 1]
**max_running_instances** | Option<**i32**> | Maximum number of instances running. This resource auto-scale based on the CPU and Memory consumption. Note: -1 means that there is no limit.  | [optional][default to 1]
**healthchecks** | [**models::Healthcheck**](Healthcheck.md) |  | 
**auto_preview** | Option<**bool**> | Specify if the environment preview option is activated or not for this application.   If activated, a preview environment will be automatically cloned at each pull request.   If not specified, it takes the value of the `auto_preview` property from the associated environment.  | [optional][default to true]
**ports** | Option<[**Vec<models::ServicePort>**](ServicePort.md)> |  | [optional]
**arguments** | Option<**Vec<String>**> |  | [optional]
**entrypoint** | Option<**String**> | optional entrypoint when launching container | [optional]
**auto_deploy** | Option<**bool**> | Specify if the application will be automatically updated after receiving a new commit. | [optional]
**annotations_groups** | Option<[**Vec<models::ServiceAnnotationRequest>**](ServiceAnnotationRequest.md)> |  | [optional]
**labels_groups** | Option<[**Vec<models::ServiceLabelRequest>**](ServiceLabelRequest.md)> |  | [optional]
**icon_uri** | Option<**String**> | Icon URI representing the application. | [optional]
**docker_target_build_stage** | Option<**String**> | The target build stage in the Dockerfile to build | [optional]
**autoscaling** | Option<[**models::KedaAutoscalingRequest**](KedaAutoscalingRequest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


