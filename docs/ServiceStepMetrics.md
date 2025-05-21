# ServiceStepMetrics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_duration_sec** | Option<**i32**> | The total duration in seconds of the service deployment or null if the deployment is not completed. | [optional]
**total_computing_duration_sec** | Option<**i32**> | The total duration in seconds of the service deployment without queuing steps. | [optional]
**details** | Option<[**Vec<models::ServiceStepMetric>**](ServiceStepMetric.md)> | A list of metrics for deployment steps of the service. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


