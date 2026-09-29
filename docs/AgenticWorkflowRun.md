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
**status** | [**models::AgenticWorkflowRunStatus**](AgenticWorkflowRunStatus.md) |  | 
**started_at** | Option<**String**> | Time the run entered RUNNING. Separate from recorded_at. Null until that transition is observed, and null for a run that reached a terminal status without it being observed. | 
**finished_at** | Option<**String**> | Time the run reached a terminal status. Null until then. | 
**duration_ms** | Option<**i64**> | finished_at minus started_at, in milliseconds. Derived, not stored. Null unless both timestamps are set. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


