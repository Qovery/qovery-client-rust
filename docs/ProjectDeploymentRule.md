# ProjectDeploymentRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | **String** | name is case insensitive | 
**description** | Option<**String**> |  | [optional]
**mode** | [**models::EnvironmentModeEnum**](EnvironmentModeEnum.md) |  | 
**cluster_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**auto_stop** | Option<**bool**> |  | [optional][default to false]
**timezone** | **String** |  | 
**start_time** | **String** |  | 
**stop_time** | **String** |  | 
**weekdays** | [**Vec<models::WeekdayEnum>**](WeekdayEnum.md) |  | 
**wildcard** | **String** | wildcard pattern composed of '?' and/or '*' used to target new created environments | [default to ]
**priority_index** | Option<**i32**> | used to select the first deployment rule to match new created environments | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


