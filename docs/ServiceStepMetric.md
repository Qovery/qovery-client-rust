# ServiceStepMetric

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**step_name** | Option<[**models::ServiceStepMetricNameEnum**](ServiceStepMetricNameEnum.md)> |  | [optional]
**status** | Option<[**models::StepMetricStatusEnum**](StepMetricStatusEnum.md)> |  | [optional]
**duration_sec** | Option<**i32**> | The duration of the step in seconds. | [optional]
**started_at** | Option<**String**> | The time at which the step started. Present while the step is ongoing and may be retained after completion. | [optional]
**sub_steps** | Option<[**Vec<models::ServiceSubStepMetric>**](ServiceSubStepMetric.md)> | Steps that ran inside this one. Only set on BUILD. Their time is already part of this step's duration: never add it to a total.  | [optional]
**builder_usage** | Option<[**models::BuilderUsage**](BuilderUsage.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


