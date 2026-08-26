# StageStepMetric

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**stage_id** | Option<**uuid::Uuid**> |  | [optional]
**step_name** | Option<[**models::StageStepMetricNameEnum**](StageStepMetricNameEnum.md)> |  | [optional]
**status** | Option<[**models::StepMetricStatusEnum**](StepMetricStatusEnum.md)> |  | [optional]
**duration_sec** | Option<**i32**> | The duration of the step in seconds. | [optional]
**started_at** | Option<**String**> | The time at which the step started. Present while the step is ongoing and may be retained after completion. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


