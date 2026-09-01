# BlueprintDeploymentStatusResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | 
**execution_id** | **String** | Engine execution identifier for this dispatch | 
**status** | **Status** | Status of the dispatch. Only the states a blueprint dispatch can reach, which is a subset of the deployment statuses used elsewhere in the API. (enum: WAITING_RUNNING, DEPLOYING, RUNNING, FAILED, CANCELING, CANCELED, INTERNAL_ERROR) | 
**started_at** | **String** |  | 
**terminated_at** | Option<**String**> | When the dispatch reached a terminal state. Null while it is still running. | 
**error_message** | Option<**String**> | The engine's failure message. Null unless the dispatch failed. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


