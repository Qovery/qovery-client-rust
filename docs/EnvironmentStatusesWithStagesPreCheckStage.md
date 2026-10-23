# EnvironmentStatusesWithStagesPreCheckStage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<[**models::StepMetricStatusEnum**](StepMetricStatusEnum.md)> |  | [optional]
**total_duration_sec** | Option<**i32**> |  | [optional]
**steps** | Option<[**Vec<models::PreCheckStepMetric>**](PreCheckStepMetric.md)> | One entry per check the pre-check ran, in order. Their time is already part of `total_duration_sec`: never add it to a total.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


