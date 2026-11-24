# AgentActivityEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actor_kind** | [**models::AgentActorKind**](AgentActorKind.md) |  | 
**policy_token_id** | Option<**uuid::Uuid**> | The policy token's id when `actor_kind` is POLICY_TOKEN and the token is known. Null otherwise. | [optional]
**at** | **String** |  | 
**user_sub** | Option<**String**> | The member's id. Null when the actor loaded no skill in the range, such as a policy token. | [optional]
**user_name** | **String** | The username the audit log records for the actor. | 
**source** | [**models::AgentActivitySource**](AgentActivitySource.md) | Where the action comes from: SKILL_API_ACTION, MCP_TOOL_CALL, POLICY_DENIAL or POLICY_TOKEN_API_ACTION. Skill loads are counted, never listed here, so SKILL_LOAD does not appear. | 
**skill_name** | Option<**String**> | The skill named in the user agent. Null when the action names none. | [optional]
**event** | **String** | The audit event type, as in `OrganizationEventType`. | 
**target_type** | **String** | The audit target type, as in `OrganizationEventTargetType`. | 
**target_name** | **String** |  | 
**project_name** | Option<**String**> |  | [optional]
**environment_name** | Option<**String**> |  | [optional]
**outcome** | [**models::AgentActionOutcome**](AgentActionOutcome.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


