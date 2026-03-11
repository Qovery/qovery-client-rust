# GhostAlertRuleResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source** | [**models::AlertRuleSource**](AlertRuleSource.md) |  | 
**name** | **String** | Name of the alert rule | 
**state** | [**models::AlertRuleState**](AlertRuleState.md) |  | 
**target** | Option<[**models::AlertTarget**](AlertTarget.md)> | May be null if target info couldn't be extracted from Prometheus | [optional]
**starts_at** | Option<**String**> | When the ghost alert started firing | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


