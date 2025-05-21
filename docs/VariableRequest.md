# VariableRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | the key of the environment variable | 
**value** | **String** | the value of the environment variable | 
**mount_path** | Option<**String**> | the path where the file will be mounted (only if type =file) | [optional]
**is_secret** | **bool** | if true, the variable will be considered as a secret and will not be accessible after its creation. Only your applications will be able to access its value at build and run time. | 
**variable_scope** | [**models::ApiVariableScopeEnum**](APIVariableScopeEnum.md) |  | 
**variable_parent_id** | [**uuid::Uuid**](uuid::Uuid.md) | based on the selected scope, it contains the ID of the service, environment or project where the variable is attached | 
**description** | Option<**String**> | optional variable description (255 characters maximum) | [optional]
**enable_interpolation_in_file** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


