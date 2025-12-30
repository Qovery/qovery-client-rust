# KedaAutoscalingResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**service_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**mode** | [**models::AutoscalingMode**](AutoscalingMode.md) |  | 
**polling_interval_seconds** | **i32** |  | 
**cooldown_period_seconds** | **i32** |  | 
**scalers** | [**Vec<models::KedaScalerResponse>**](KedaScalerResponse.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


