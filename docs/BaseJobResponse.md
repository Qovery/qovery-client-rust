# BaseJobResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**environment** | [**models::ReferenceObject**](ReferenceObject.md) |  | 
**maximum_cpu** | **i32** | Maximum cpu that can be allocated to the job based on organization cluster configuration. unit is millicores (m). 1000m = 1 cpu | 
**maximum_memory** | **i32** | Maximum memory that can be allocated to the job based on organization cluster configuration. unit is MB. 1024 MB = 1GB | 
**name** | **String** | name is case insensitive | 
**description** | Option<**String**> |  | [optional]
**cpu** | **i32** | unit is millicores (m). 1000m = 1 cpu | 
**memory** | **i32** | unit is MB. 1024 MB = 1GB | 
**max_nb_restart** | Option<**i32**> | Maximum number of restart allowed before the job is considered as failed 0 means that no restart/crash of the job is allowed  | [optional]
**max_duration_seconds** | Option<**i32**> | Maximum number of seconds allowed for the job to run before killing it and mark it as failed  | [optional]
**auto_preview** | **bool** | Indicates if the 'environment preview option' is enabled for this container.   If enabled, a preview environment will be automatically cloned when `/preview` endpoint is called.   If not specified, it takes the value of the `auto_preview` property from the associated environment.  | 
**port** | Option<**i32**> | Port where to run readiness and liveliness probes checks. The port will not be exposed externally | [optional]
**source** | [**models::BaseJobResponseAllOfSource**](BaseJobResponse_allOf_source.md) |  | 
**healthchecks** | [**models::Healthcheck**](Healthcheck.md) |  | 
**auto_deploy** | Option<**bool**> | Specify if the job will be automatically updated after receiving a new image tag or a new commit according to the source type.  The new image tag shall be communicated via the \"Auto Deploy job\" endpoint https://api-doc.qovery.com/#tag/Jobs/operation/autoDeployJobEnvironments  | [optional]
**icon_uri** | **String** | Icon URI representing the job. | 
**service_type** | [**models::ServiceTypeEnum**](ServiceTypeEnum.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


