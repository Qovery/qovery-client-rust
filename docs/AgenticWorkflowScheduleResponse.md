# AgenticWorkflowScheduleResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cron_expression** | **String** |  | 
**timezone** | **String** | tz database identifier the expression is evaluated in. | 
**next_run_at** | Option<**String**> | When the schedule fires next. Null while the workflow is disabled, since a disabled workflow accumulates no occurrences. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


