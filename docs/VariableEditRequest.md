# VariableEditRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | the key of the environment variable | 
**value** | Option<**String**> | the value of the environment variable | [optional]
**description** | Option<**String**> | optional variable description (255 characters maximum) | [optional]
**enable_interpolation_in_file** | Option<**bool**> |  | [optional]
**secret_manager_access_id** | Option<**uuid::Uuid**> |  | [optional]
**json_target_key** | Option<**String**> | Optional JSON member/index path, only supported for `EXTERNAL_SECRET` variables (not external secret files). Use dots for nested members and numeric indexes for arrays, for example `accounts.0.password`. Escape literal dots with `\\.` and backslashes with `\\\\`; other query characters are treated literally. Members must be non-empty, with no leading or trailing whitespace, control characters, or unescaped trailing backslash. Omit or set to null to clear JSON selection and read the whole secret. Send the current selector on unrelated edits to preserve it. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


