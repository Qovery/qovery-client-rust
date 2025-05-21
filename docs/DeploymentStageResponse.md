# DeploymentStageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**environment** | [**models::ReferenceObject**](ReferenceObject.md) |  | 
**name** | Option<**String**> | name is case insensitive | [optional]
**description** | Option<**String**> |  | [optional]
**deployment_order** | Option<**i32**> | Position of the deployment stage within the environment | [optional]
**services** | Option<[**Vec<models::DeploymentStageServiceResponse>**](DeploymentStageServiceResponse.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


