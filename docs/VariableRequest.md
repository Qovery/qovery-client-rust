# VariableRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | the key of the environment variable | 
**value** | **String** | the value of the environment variable | 
**mount_path** | Option<**String**> | the path where the file will be mounted (only if type =file) | [optional]
**is_secret** | **bool** | if true, the variable will be considered as a secret and will not be accessible after its creation. Only your applications will be able to access its value at build and run time. | 
**variable_scope** | [**models::ApiVariableScopeEnum**](APIVariableScopeEnum.md) |  | 
**variable_parent_id** | **uuid::Uuid** | based on the selected scope, it contains the ID of the service, environment or project where the variable is attached | 
**description** | Option<**String**> | optional variable description (255 characters maximum) | [optional]
**enable_interpolation_in_file** | Option<**bool**> |  | [optional]
**secret_manager_access_id** | Option<**uuid::Uuid**> |  | [optional]
**override_if_exists** | Option<**bool**> | If true and a variable with the same key already exists at a higher scope (project or environment), an override of that variable is created instead of failing with 409. The override follows the usual rules: same secret kind, target must be a value or file variable, strictly lower scope. Defaults to false. | [optional][default to false]
**json_target_key** | Option<**String**> | Optional JSON member/index path, only supported for `EXTERNAL_SECRET` variables (not external secret files). Use dots for nested members and numeric indexes for arrays, for example `accounts.0.password`. Escape literal dots with `\\.` and backslashes with `\\\\`; other query characters are treated literally. Members must be non-empty, with no leading or trailing whitespace, control characters, or unescaped trailing backslash. Omit or set to null to read the whole secret. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


