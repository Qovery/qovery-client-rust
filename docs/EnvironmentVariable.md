# EnvironmentVariable

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**key** | **String** | key is case sensitive. | 
**value** | Option<**String**> | value of the env variable. | [optional]
**mount_path** | Option<**String**> | should be set for file only. variable mount path makes variable a file (where file should be mounted). | [optional]
**description** | Option<**String**> | optional variable description (255 characters maximum) | [optional]
**enable_interpolation_in_file** | Option<**bool**> |  | [optional]
**overridden_variable** | Option<[**models::EnvironmentVariableOverride**](EnvironmentVariableOverride.md)> |  | [optional]
**aliased_variable** | Option<[**models::EnvironmentVariableAlias**](EnvironmentVariableAlias.md)> |  | [optional]
**scope** | [**models::ApiVariableScopeEnum**](APIVariableScopeEnum.md) |  | 
**variable_type** | [**models::ApiVariableTypeEnum**](APIVariableTypeEnum.md) |  | 
**service_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**service_name** | Option<**String**> |  | [optional]
**service_type** | Option<[**models::LinkedServiceTypeEnum**](LinkedServiceTypeEnum.md)> |  | [optional]
**owned_by** | Option<**String**> | Entity that created/own the variable (i.e: Qovery, Doppler) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


