# VariableOverrideRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**value** | **String** | the value to be used as Override of the targeted environment variable. | 
**override_scope** | [**models::ApiVariableScopeEnum**](APIVariableScopeEnum.md) |  | 
**override_parent_id** | [**uuid::Uuid**](uuid::Uuid.md) | the id of the variable that is aliased. | 
**description** | Option<**String**> | optional variable description (255 characters maximum) | [optional]
**enable_interpolation_in_file** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


