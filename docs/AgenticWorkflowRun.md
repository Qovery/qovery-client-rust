# AgenticWorkflowRun

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Run ID. | 
**source_workflow_id** | **uuid::Uuid** | ID of the workflow the run was requested for. A CLONE_ENVIRONMENT run executes as a fresh clone carrying its own ID, which run history does not report, so this is never the ID of the workflow that actually executed. | 
**trigger** | [**models::AgenticWorkflowRunTrigger**](AgenticWorkflowRunTrigger.md) |  | 
**prompt** | Option<**String**> | Agent prompt captured when the run was requested. It is a snapshot, so later edits to the workflow do not change it. Null when the workflow had no prompt. | 
**created_at** | **String** | Time the run was requested. | 
**recorded_at** | Option<**String**> | Time the run was registered in run history, shortly after it was requested. This is not a lifecycle start time: nothing reports when the agent itself started, so this value must not be used to measure a run. Null when it is unknown. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


