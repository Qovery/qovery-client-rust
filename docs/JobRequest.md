# JobRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | name is case insensitive | 
**description** | Option<**String**> |  | [optional]
**cpu** | Option<**i32**> | unit is millicores (m). 1000m = 1 cpu | [optional][default to 500]
**memory** | Option<**i32**> | unit is MB. 1024 MB = 1GB | [optional][default to 512]
**max_nb_restart** | Option<**i32**> | Maximum number of restart allowed before the job is considered as failed 0 means that no restart/crash of the job is allowed  | [optional][default to 0]
**max_duration_seconds** | Option<**i32**> | Maximum number of seconds allowed for the job to run before killing it and mark it as failed  | [optional]
**auto_preview** | Option<**bool**> | Indicates if the 'environment preview option' is enabled for this container.   If enabled, a preview environment will be automatically cloned when `/preview` endpoint is called.   If not specified, it takes the value of the `auto_preview` property from the associated environment.  | [optional]
**port** | Option<**i32**> | Port where to run readiness and liveliness probes checks. The port will not be exposed externally | [optional]
**source** | Option<[**models::JobRequestAllOfSource**](JobRequest_allOf_source.md)> |  | [optional]
**healthchecks** | [**models::Healthcheck**](Healthcheck.md) |  | 
**schedule** | Option<[**models::JobRequestAllOfSchedule**](JobRequest_allOf_schedule.md)> |  | [optional]
**auto_deploy** | Option<**bool**> | Specify if the job will be automatically updated after receiving a new image tag or a new commit according to the source type.  The new image tag shall be communicated via the \"Auto Deploy job\" endpoint https://api-doc.qovery.com/#tag/Jobs/operation/autoDeployJobEnvironments  | [optional]
**annotations_groups** | Option<[**Vec<models::ServiceAnnotationRequest>**](ServiceAnnotationRequest.md)> |  | [optional]
**labels_groups** | Option<[**Vec<models::ServiceLabelRequest>**](ServiceLabelRequest.md)> |  | [optional]
**icon_uri** | Option<**String**> | Icon URI representing the job. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


