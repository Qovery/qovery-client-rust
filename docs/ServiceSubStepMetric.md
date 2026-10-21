# ServiceSubStepMetric

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | PROVISION_BUILDER, IMAGE_PUSH or CACHE_EXPORT. A string, not an enum, so a new sub-step never breaks a client: accept values not listed here.  | 
**status** | [**models::StepMetricStatusEnum**](StepMetricStatusEnum.md) |  | 
**duration_sec** | **i32** | The duration of the sub-step in seconds. | 
**started_at** | Option<**String**> | The time at which the sub-step started. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


