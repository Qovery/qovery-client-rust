# TerraformDeploymentRestrictionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**mode** | [**models::DeploymentRestrictionModeEnum**](DeploymentRestrictionModeEnum.md) |  | 
**r#type** | [**models::DeploymentRestrictionTypeEnum**](DeploymentRestrictionTypeEnum.md) |  | 
**value** | **String** | ‘For `PATH` restrictions, the value must not start with `/`’ | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


