# EnvironmentDeploymentRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**on_demand_preview** | Option<**bool**> |  | [optional][default to false]
**auto_stop** | Option<**bool**> |  | [optional][default to false]
**auto_preview** | Option<**bool**> |  | [optional][default to false]
**timezone** | **String** |  | 
**start_time** | **String** |  | 
**stop_time** | **String** |  | 
**weekdays** | [**Vec<models::WeekdayEnum>**](WeekdayEnum.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


