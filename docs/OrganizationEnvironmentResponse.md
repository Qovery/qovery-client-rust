# OrganizationEnvironmentResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | **String** | name is case insensitive | 
**organization** | [**models::ReferenceObject**](ReferenceObject.md) |  | 
**project** | [**models::ReferenceObject**](ReferenceObject.md) |  | 
**mode** | [**models::EnvironmentModeEnum**](EnvironmentModeEnum.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


