# Environment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | **String** | name is case insensitive | 
**organization** | [**models::ReferenceObject**](ReferenceObject.md) |  | 
**project** | [**models::ReferenceObject**](ReferenceObject.md) |  | 
**last_updated_by** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | uuid of the user that made the last update | [optional]
**cloud_provider** | [**models::EnvironmentAllOfCloudProvider**](Environment_allOf_cloud_provider.md) |  | 
**mode** | [**models::EnvironmentModeEnum**](EnvironmentModeEnum.md) |  | 
**cluster_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**cluster_name** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


