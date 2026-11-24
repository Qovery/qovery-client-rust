# AgentActivityByMember

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actor_kind** | [**models::AgentActorKind**](AgentActorKind.md) |  | 
**policy_token_id** | Option<**uuid::Uuid**> | The policy token's id when `actor_kind` is POLICY_TOKEN and the token is known. Null otherwise. | [optional]
**user_sub** | Option<**String**> | The member's id, as returned by \"List organization members\". Null for an actor that loaded no skill in the range, such as a policy token. | [optional]
**user_name** | **String** | The username the audit log records for the actor. | 
**skill_loads** | **i64** |  | 
**actions** | **i64** |  | 
**denials** | **i64** |  | 
**top_skills** | **Vec<String>** | The member's most used skills, at most 3, most used first. | 
**last_activity_at** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


