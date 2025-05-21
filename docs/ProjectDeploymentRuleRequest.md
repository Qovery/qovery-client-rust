# ProjectDeploymentRuleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
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

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


