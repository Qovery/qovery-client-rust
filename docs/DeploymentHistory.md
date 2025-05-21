# DeploymentHistory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | Option<**String**> | name of the service | [optional]
**commit** | Option<[**models::Commit**](Commit.md)> |  | [optional]
**status** | Option<[**models::DeploymentHistoryStatusEnum**](DeploymentHistoryStatusEnum.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


