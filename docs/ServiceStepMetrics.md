# ServiceStepMetrics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_duration_sec** | Option<**i32**> | The total duration in seconds of the service deployment or null if the deployment is not completed. | 
**total_computing_duration_sec** | **i32** | The total duration in seconds of the service deployment without queuing steps. | 
**details** | [**Vec<models::ServiceStepMetric>**](ServiceStepMetric.md) | A list of metrics for deployment steps of the service. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


