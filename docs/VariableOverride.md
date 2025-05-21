# VariableOverride

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | The id of the overriden variable | 
**key** | **String** | The key of the overriden variable | 
**value** | Option<**String**> | The value of the overriden variable | [optional]
**mount_path** | **String** | The mounth path of the overriden variable (only if environment variable type is 'file') | 
**scope** | [**models::ApiVariableScopeEnum**](APIVariableScopeEnum.md) |  | 
**variable_type** | [**models::ApiVariableTypeEnum**](APIVariableTypeEnum.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


