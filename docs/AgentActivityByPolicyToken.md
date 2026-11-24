# AgentActivityByPolicyToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**policy_token_id** | Option<**uuid::Uuid**> | The token's id. Null groups older role refusals recorded before refusals named the token: a policy token, but not which one. | [optional]
**token_name** | Option<**String**> | The token's current name, or its last audited name when `deleted` is true. Null when unknown. | [optional]
**deleted** | **bool** | Whether the token no longer exists. | 
**role_id** | Option<**uuid::Uuid**> | The token's current role. Null when the token is deleted or unknown. | [optional]
**role_name** | Option<**String**> | The name of the token's current role. Null when the token is deleted or unknown. | [optional]
**expires_at** | Option<**String**> | When the token expires. Null when it never expires, or when it is deleted or unknown. | [optional]
**skill_loads** | **i64** |  | 
**actions** | **i64** |  | 
**denials** | **i64** |  | 
**last_activity_at** | **String** |  | 
**top_actions** | [**Vec<models::AgentActionsByType>**](AgentActionsByType.md) | The token's most frequent actions, at most 3, most frequent first. | 
**top_skills** | **Vec<String>** | The token's most used skills, at most 3, most used first. | 
**timeline** | [**Vec<models::AgentActivityTokenPoint>**](AgentActivityTokenPoint.md) | The token's actions per bucket of the requested size, oldest first, only for buckets where it acted. Filled when `bucket` is DAY, or when `policyTokenId` is set (so HOUR buckets need `policyTokenId`). Empty otherwise. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


