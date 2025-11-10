# AlertRuleResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Organization identifier | 
**cluster_id** | [**uuid::Uuid**](uuid::Uuid.md) |  Cluster identifier | 
**name** | **String** | Name of the alert rule  | 
**description** | **String** | Description of what the alert monitors | 
**promql_expr** | **String** | PromQL expression to evaluate | 
**for_duration** | **String** | Duration the condition must be true before firing (ISO-8601 duration format) | 
**severity** | [**models::AlertSeverity**](AlertSeverity.md) |  | 
**enabled** | **bool** | Whether the alert rule is enabled | 
**alert_receiver_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | List of alert receiver IDs to send notifications to | 
**presentation** | [**models::AlertPresentationResponse**](AlertPresentationResponse.md) |  | 
**target** | [**models::AlertTarget**](AlertTarget.md) |  | 
**state** | [**models::AlertRuleState**](AlertRuleState.md) |  | 
**is_up_to_date** | **bool** | Indicates whether the current version of the alert has been synced with the alerting system. If false, an outdated version is currently deployed. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


