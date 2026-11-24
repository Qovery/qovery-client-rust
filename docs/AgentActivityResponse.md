# AgentActivityResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from** | **String** | Start of the range actually used, after defaults. | 
**to** | **String** | End of the range actually used, after defaults. | 
**bucket** | [**models::SkillUsageBucket**](SkillUsageBucket.md) |  | 
**coverage** | [**models::AgentActivityCoverage**](AgentActivityCoverage.md) |  | 
**totals** | [**models::AgentActivityTotals**](AgentActivityTotals.md) |  | 
**timeline** | [**Vec<models::AgentActivityPoint>**](AgentActivityPoint.md) | Activity per hour or per day, oldest first. Buckets are cut in UTC; empty buckets are omitted. | 
**actions_by_type** | [**Vec<models::AgentActionsByType>**](AgentActionsByType.md) | Agent actions per audit event and target type, most frequent first, at most 200 rows. | 
**by_member** | [**Vec<models::AgentActivityByMember>**](AgentActivityByMember.md) | Activity per member, most active first, at most 500 rows. | 
**by_skill** | [**Vec<models::AgentActivityBySkill>**](AgentActivityBySkill.md) | Activity per skill, most active first, at most 100 rows. | 
**by_policy_token** | [**Vec<models::AgentActivityByPolicyToken>**](AgentActivityByPolicyToken.md) | Activity per policy token, most active first, at most 100 rows. Always empty on `getMyAgentActivity`. | 
**recent** | [**Vec<models::AgentActivityEntry>**](AgentActivityEntry.md) | The 100 most recent agent actions, newest first. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


