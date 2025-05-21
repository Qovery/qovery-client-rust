# DeploymentHistoryEnvironment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**status** | Option<[**models::StateEnum**](StateEnum.md)> |  | [optional]
**origin** | Option<[**models::OrganizationEventOrigin**](OrganizationEventOrigin.md)> |  | [optional]
**triggered_by** | Option<**String**> |  | [optional]
**applications** | Option<[**Vec<models::DeploymentHistoryApplication>**](DeploymentHistoryApplication.md)> |  | [optional]
**containers** | Option<[**Vec<models::DeploymentHistoryContainer>**](DeploymentHistoryContainer.md)> |  | [optional]
**databases** | Option<[**Vec<models::DeploymentHistoryDatabase>**](DeploymentHistoryDatabase.md)> |  | [optional]
**jobs** | Option<[**Vec<models::DeploymentHistoryJobResponse>**](DeploymentHistoryJobResponse.md)> |  | [optional]
**helms** | Option<[**Vec<models::DeploymentHistoryHelmResponse>**](DeploymentHistoryHelmResponse.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


