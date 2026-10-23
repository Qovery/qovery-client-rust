# PreCheckStepMetric

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | CLOUD_CREDENTIALS_CHECK (once per deployment) or GIT_CHECK (once per service built or fetched from Git). A string, not an enum, so a new check never breaks a client: accept values not listed here. Log lines of the check carry the same value in `step_name`.  | 
**status** | [**models::StepMetricStatusEnum**](StepMetricStatusEnum.md) |  | 
**duration_sec** | **i32** | The duration of the check in seconds, rounded down (0 means under a second). | 
**started_at** | Option<**String**> | The time at which the check started. | [optional]
**service_id** | Option<**uuid::Uuid**> | The service whose Git configuration was checked. Only set on GIT_CHECK. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


