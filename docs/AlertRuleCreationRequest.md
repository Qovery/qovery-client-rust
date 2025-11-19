# AlertRuleCreationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization identifier | 
**cluster_id** | [**uuid::Uuid**](uuid::Uuid.md) |  Cluster identifier where the rule will be deployed | 
**name** | **String** | Name of the alert rule | 
**description** | **String** | Description of what the alert monitors  | 
**tag** | **String** |  | 
**condition** | [**models::AlertRuleCondition**](AlertRuleCondition.md) |  | 
**for_duration** | **String** | Duration the condition must be true before firing (ISO-8601 duration format) | 
**severity** | [**models::AlertSeverity**](AlertSeverity.md) |  | 
**presentation** | [**models::AlertPresentation**](AlertPresentation.md) |  | 
**enabled** | **bool** | Whether the alert rule is enabled | 
**alert_receiver_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | List of alert receiver IDs to send notifications to | 
**target** | [**models::AlertTarget**](AlertTarget.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


