# KedaScalerResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**scaler_type** | **String** |  | 
**enabled** | **bool** |  | 
**role** | [**models::KedaScalerRole**](KedaScalerRole.md) |  | 
**config_json** | Option<[**serde_json::Value**](.md)> |  | [optional]
**config_yaml** | Option<**String**> |  | [optional]
**trigger_authentication** | Option<[**models::KedaTriggerAuthenticationResponse**](KedaTriggerAuthenticationResponse.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


