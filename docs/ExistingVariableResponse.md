# ExistingVariableResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | 
**key** | **String** |  | 
**scope** | [**models::ApiVariableScopeEnum**](APIVariableScopeEnum.md) |  | 
**organization_id** | Option<**uuid::Uuid**> |  | [optional]
**project_id** | Option<**uuid::Uuid**> |  | [optional]
**environment_id** | Option<**uuid::Uuid**> |  | [optional]
**service_id** | Option<**uuid::Uuid**> |  | [optional]
**variable_type** | [**models::ApiVariableTypeEnum**](APIVariableTypeEnum.md) |  | 
**is_secret** | **bool** |  | 
**mount_path** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


