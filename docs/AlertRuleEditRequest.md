# AlertRuleEditRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the alert rule | 
**description** | **String** | Description of what the alert monitors | 
**tag** | **String** |  | 
**condition** | [**models::AlertRuleCondition**](AlertRuleCondition.md) |  | 
**for_duration** | **String** | Duration the condition must be true before firing (ISO-8601 duration format) | 
**severity** | [**models::AlertSeverity**](AlertSeverity.md) |  | 
**enabled** | **bool** | Whether the alert rule is enabled | 
**alert_receiver_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | List of alert receiver IDs to send notifications to | 
**presentation** | [**models::AlertPresentation**](AlertPresentation.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


