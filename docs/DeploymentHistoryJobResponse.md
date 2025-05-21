# DeploymentHistoryJobResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | Option<**String**> | name of the job | [optional]
**status** | Option<[**models::StateEnum**](StateEnum.md)> |  | [optional]
**image_name** | Option<**String**> |  | [optional]
**tag** | Option<**String**> |  | [optional]
**commit** | Option<[**models::Commit**](Commit.md)> |  | [optional]
**schedule** | Option<[**models::DeploymentHistoryJobResponseAllOfSchedule**](DeploymentHistoryJobResponse_allOf_schedule.md)> |  | [optional]
**arguments** | Option<**Vec<String>**> |  | [optional]
**entrypoint** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


