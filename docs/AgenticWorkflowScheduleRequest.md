# AgenticWorkflowScheduleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cron_expression** | **String** | Five-field cron expression, the same syntax Kubernetes cron jobs accept. Rejected if it fires more than once every 5 minutes, since each run deploys a full environment. | 
**timezone** | **String** | tz database identifier the expression is evaluated in. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


