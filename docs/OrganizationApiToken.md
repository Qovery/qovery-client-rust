# OrganizationApiToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**role_name** | Option<**String**> |  | [optional]
**role_id** | Option<**uuid::Uuid**> |  | [optional]
**last_used_at** | Option<**String**> | Time when token usage was last recorded. Updated periodically, so recent usage may not appear immediately. Null when no usage has been recorded. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


