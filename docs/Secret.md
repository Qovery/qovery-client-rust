# Secret

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**key** | **String** | key is case sensitive | 
**overridden_secret** | Option<[**models::SecretOverride**](SecretOverride.md)> |  | [optional]
**aliased_secret** | Option<[**models::SecretAlias**](SecretAlias.md)> |  | [optional]
**scope** | [**models::ApiVariableScopeEnum**](APIVariableScopeEnum.md) |  | 
**variable_type** | Option<[**models::ApiVariableTypeEnum**](APIVariableTypeEnum.md)> |  | [optional]
**service_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**service_name** | Option<**String**> |  | [optional]
**service_type** | Option<[**models::LinkedServiceTypeEnum**](LinkedServiceTypeEnum.md)> |  | [optional]
**owned_by** | Option<**String**> | Entity that created/own the variable (i.e: Qovery, Doppler) | [optional]
**description** | Option<**String**> | optional variable description (255 characters maximum) | [optional]
**enable_interpolation_in_file** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


