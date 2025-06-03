# EnvironmentLog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**created_at** | **String** |  | 
**scope** | Option<[**models::EnvironmentLogScope**](EnvironmentLog_scope.md)> |  | [optional]
**state** | Option<[**models::StatusKindEnum**](StatusKindEnum.md)> |  | [optional]
**message** | **String** | Log message | 
**execution_id** | Option<**String**> | Only for errors. Helps Qovery team to investigate. | [optional]
**hint** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


