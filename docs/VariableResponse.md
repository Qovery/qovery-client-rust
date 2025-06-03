# VariableResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**key** | **String** |  | 
**value** | Option<**String**> |  | 
**mount_path** | Option<**String**> |  | [optional]
**overridden_variable** | Option<[**models::VariableOverride**](VariableOverride.md)> |  | [optional]
**aliased_variable** | Option<[**models::VariableAlias**](VariableAlias.md)> |  | [optional]
**scope** | [**models::ApiVariableScopeEnum**](APIVariableScopeEnum.md) |  | 
**variable_type** | [**models::ApiVariableTypeEnum**](APIVariableTypeEnum.md) |  | 
**service_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The id of the service referenced by this variable. | [optional]
**service_name** | Option<**String**> | The name of the service referenced by this variable. | [optional]
**service_type** | Option<[**models::LinkedServiceTypeEnum**](LinkedServiceTypeEnum.md)> |  | [optional]
**owned_by** | Option<**String**> | Entity that created/own the variable (i.e: Qovery, Doppler) | [optional]
**is_secret** | **bool** |  | 
**description** | Option<**String**> |  | [optional]
**enable_interpolation_in_file** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


