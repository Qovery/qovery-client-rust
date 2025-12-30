# Application

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**storage** | Option<[**Vec<models::ServiceStorageStorageInner>**](ServiceStorage_storage_inner.md)> |  | [optional]
**environment** | [**models::ReferenceObject**](ReferenceObject.md) |  | 
**git_repository** | Option<[**models::ApplicationGitRepository**](ApplicationGitRepository.md)> |  | [optional]
**maximum_cpu** | Option<**i32**> | Maximum cpu that can be allocated to the application based on organization cluster configuration. unit is millicores (m). 1000m = 1 cpu | [optional]
**maximum_memory** | Option<**i32**> | Maximum memory that can be allocated to the application based on organization cluster configuration. unit is MB. 1024 MB = 1GB | [optional]
**maximun_gpu** | Option<**i32**> |  | [optional][default to 0]
**name** | **String** | name is case insensitive | 
**description** | Option<**String**> | give a description to this application | [optional]
**build_mode** | Option<[**models::BuildModeEnum**](BuildModeEnum.md)> |  | [optional]
**dockerfile_path** | Option<**String**> | The path of the associated Dockerfile. Only if you are using build_mode = DOCKER | [optional]
**cpu** | Option<**i32**> | unit is millicores (m). 1000m = 1 cpu | [optional]
**memory** | Option<**i32**> | unit is MB. 1024 MB = 1GB | [optional]
**gpu** | Option<**i32**> |  | [optional][default to 0]
**min_running_instances** | Option<**i32**> | Minimum number of instances running. This resource auto-scale based on the CPU and Memory consumption. Note: 0 means that there is no application running.  | [optional][default to 1]
**max_running_instances** | Option<**i32**> | Maximum number of instances running. This resource auto-scale based on the CPU and Memory consumption. Note: -1 means that there is no limit.  | [optional][default to 1]
**healthchecks** | [**models::Healthcheck**](Healthcheck.md) |  | 
**auto_preview** | Option<**bool**> | Specify if the environment preview option is activated or not for this application.   If activated, a preview environment will be automatically cloned at each pull request.   If not specified, it takes the value of the `auto_preview` property from the associated environment.  | [optional][default to true]
**ports** | Option<[**Vec<models::ServicePort>**](ServicePort.md)> |  | [optional]
**arguments** | Option<**Vec<String>**> |  | [optional]
**entrypoint** | Option<**String**> | optional entrypoint when launching container | [optional]
**auto_deploy** | Option<**bool**> | Specify if the application will be automatically updated after receiving a new commit. | [optional]
**annotations_groups** | Option<[**Vec<models::OrganizationAnnotationsGroupResponse>**](OrganizationAnnotationsGroupResponse.md)> |  | [optional]
**labels_groups** | Option<[**Vec<models::OrganizationLabelsGroupResponse>**](OrganizationLabelsGroupResponse.md)> |  | [optional]
**icon_uri** | **String** | Icon URI representing the application. | 
**service_type** | [**models::ServiceTypeEnum**](ServiceTypeEnum.md) |  | 
**docker_target_build_stage** | Option<**String**> | The target build stage in the Dockerfile to build | [optional]
**autoscaling** | Option<[**models::KedaAutoscalingResponse**](KedaAutoscalingResponse.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


