# DeploymentHistoryHelmResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | Option<**String**> | name of the helm | [optional]
**status** | Option<[**models::StateEnum**](StateEnum.md)> |  | [optional]
**commit** | Option<[**models::Commit**](Commit.md)> |  | [optional]
**repository** | Option<[**models::DeploymentHistoryHelmResponseAllOfRepository**](DeploymentHistoryHelmResponse_allOf_repository.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


