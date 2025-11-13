# OrganizationEventResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**timestamp** | Option<**String**> |  | [optional]
**event_type** | Option<[**models::OrganizationEventType**](OrganizationEventType.md)> |  | [optional]
**target_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**target_name** | Option<**String**> |  | [optional]
**target_type** | Option<[**models::OrganizationEventTargetType**](OrganizationEventTargetType.md)> |  | [optional]
**sub_target_type** | Option<[**models::OrganizationEventSubTargetType**](OrganizationEventSubTargetType.md)> |  | [optional]
**change** | Option<**String**> |  | [optional]
**origin** | Option<[**models::OrganizationEventOrigin**](OrganizationEventOrigin.md)> |  | [optional]
**triggered_by** | Option<**String**> |  | [optional]
**project_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**project_name** | Option<**String**> |  | [optional]
**environment_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**environment_name** | Option<**String**> |  | [optional]
**user_agent** | Option<**String**> |  | [optional]
**original_change** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


