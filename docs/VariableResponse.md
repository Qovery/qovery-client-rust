# VariableResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**key** | **String** |  | 
**value** | Option<**String**> |  | 
**mount_path** | Option<**String**> |  | [optional]
**overridden_variable** | Option<[**models::VariableOverride**](VariableOverride.md)> |  | [optional]
**aliased_variable** | Option<[**models::VariableAlias**](VariableAlias.md)> |  | [optional]
**scope** | [**models::ApiVariableScopeEnum**](APIVariableScopeEnum.md) |  | 
**variable_type** | [**models::ApiVariableTypeEnum**](APIVariableTypeEnum.md) |  | 
**service_id** | Option<**uuid::Uuid**> | The id of the service referenced by this variable. | [optional]
**service_name** | Option<**String**> | The name of the service referenced by this variable. | [optional]
**service_type** | Option<[**models::LinkedServiceTypeEnum**](LinkedServiceTypeEnum.md)> |  | [optional]
**owned_by** | Option<**String**> | Entity that created/own the variable (i.e: Qovery, Doppler) | [optional]
**is_created_by_qovery** | Option<**bool**> | True when Qovery created the variable on its own, such as the alias it adds for each output of a job, Terraform service or blueprint. False for variables a user created and for built-in variables, which `variable_type` already identifies. `owned_by` does not tell these apart. It stays true after a user edits the variable. Clients that sync variables, such as the Terraform provider, can leave these variables out. | [optional][readonly]
**is_secret** | **bool** |  | 
**description** | Option<**String**> |  | [optional]
**enable_interpolation_in_file** | Option<**bool**> |  | [optional]
**secret_manager_access_id** | Option<**uuid::Uuid**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


