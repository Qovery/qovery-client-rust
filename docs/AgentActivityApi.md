# \AgentActivityApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_agent_activity**](AgentActivityApi.md#get_agent_activity) | **GET** /organization/{organizationId}/agentActivity | Get agent activity
[**get_my_agent_activity**](AgentActivityApi.md#get_my_agent_activity) | **GET** /organization/{organizationId}/agentActivity/me | Get my agent activity



## get_agent_activity

> models::AgentActivityResponse get_agent_activity(organization_id, from, to, bucket, user_sub, skill_name, actor_kind, policy_token_id, outcome, source)
Get agent activity

Return what members' coding agents did on Qovery over a time range, linked to the skills they loaded: skill loads (agent loads only, a person's VIEW is not counted), API calls made from a Qovery skill, Qovery MCP server tool calls, every audited call made by a policy token, and requests refused by a policy or by role permissions. A refusal by a policy token's policy (401) and a refusal by role permissions (403) both count as DENIED. Activity is shown by member, by skill and by policy token. Requires the MANAGE_ORGANIZATION_DATA permission (organization owner or admin); members read their own activity with `getMyAgentActivity`.  Every filter is optional and they combine. `policyTokenId` implies `actorKind` POLICY_TOKEN, and neither combines with `userSub`: those combinations, an unknown enum value or a malformed id are refused with 400.  Coverage: only actions taken through Qovery (its API, CLI, skills and MCP server) are visible. What an agent does elsewhere, such as in a shell, in git or in a cloud console, is not. The response repeats this in `coverage`.  Attribution is indicative, not proof. A member is matched on the username the audit log records, and an action is attributed to a skill from the `QoverySkill/<skill>` user agent the client sends: a client can claim a skill it does not run, or leave the user agent out. What an agent may actually do is enforced by its token's policy, and every refusal is recorded.  No request content is returned: only what the audit log shows (event, target, project and environment names, time, source), never prompt text or request bodies.  The range defaults to the 30 days ending now and spans at most 90 days. `by_member` holds at most 500 rows, `by_skill` 100, `by_policy_token` 100, `actions_by_type` 200 and `recent` the 100 most recent agent actions. Buckets are cut in UTC; empty buckets are omitted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**from** | Option<**String**> | Start of the range, inclusive. Defaults to 30 days before `to`. |  |
**to** | Option<**String**> | End of the range, exclusive. Defaults to now. Must be after `from`, and the range must not exceed 90 days, otherwise the request is refused with 400. |  |
**bucket** | Option<**String**> | Width of each point of the timeline. Defaults to DAY. |  |[default to DAY]
**user_sub** | Option<**String**> | Only show this member, by the id returned by \"List organization members\". The member is matched on the username of their most recent skill load, so a member with no skill load in the last 90 days shows no action. |  |
**skill_name** | Option<**String**> | Only count the activity attributed to this skill. Lowercase letters, digits, `.`, `_` or `-`, at most 64 characters, otherwise the request is refused with 400. |  |
**actor_kind** | Option<[**AgentActorKind**](AgentActorKind.md)> | Only count members (MEMBER) or policy tokens (POLICY_TOKEN). Cannot be combined with `userSub` when POLICY_TOKEN, otherwise the request is refused with 400. |  |
**policy_token_id** | Option<**uuid::Uuid**> | Only count this policy token, by its id. Implies `actorKind` POLICY_TOKEN. Cannot be combined with `actorKind` MEMBER or with `userSub`, otherwise the request is refused with 400. With `bucket` HOUR, it is required to fill the token's `timeline` in `by_policy_token`. |  |
**outcome** | Option<[**AgentActionOutcome**](AgentActionOutcome.md)> | DENIED lists refusals only, and leaves skill loads out. ALLOWED lists everything else, skill loads included. |  |
**source** | Option<[**AgentActivitySource**](AgentActivitySource.md)> | Only count the activity from this source. |  |

### Return type

[**models::AgentActivityResponse**](AgentActivityResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_agent_activity

> models::AgentActivityResponse get_my_agent_activity(organization_id, from, to, bucket, skill_name, outcome, source)
Get my agent activity

Return the caller's own agent activity: exactly what admins see about them through `getAgentActivity`, and nothing about other members. Any organization member can call it (READ_ORGANIZATION_DATA). Policy tokens belong to the organization, so this view takes no `actorKind` or `policyTokenId` and its `by_policy_token` is always empty. A refusal by policy (401) or by role permissions (403) counts as DENIED. API and policy tokens cannot call it (400): they are not members.  Coverage: only actions taken through Qovery (its API, CLI, skills and MCP server) are visible. What an agent does elsewhere, such as in a shell, in git or in a cloud console, is not. The response repeats this in `coverage`.  Attribution is indicative, not proof. A member is matched on the username the audit log records, and an action is attributed to a skill from the `QoverySkill/<skill>` user agent the client sends: a client can claim a skill it does not run, or leave the user agent out. What an agent may actually do is enforced by its token's policy, and every refusal is recorded.  No request content is returned: only what the audit log shows (event, target, project and environment names, time, source), never prompt text or request bodies.  The range defaults to the 30 days ending now and spans at most 90 days. `by_member` holds at most 500 rows, `by_skill` 100, `by_policy_token` 100, `actions_by_type` 200 and `recent` the 100 most recent agent actions. Buckets are cut in UTC; empty buckets are omitted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**from** | Option<**String**> | Start of the range, inclusive. Defaults to 30 days before `to`. |  |
**to** | Option<**String**> | End of the range, exclusive. Defaults to now. Must be after `from`, and the range must not exceed 90 days, otherwise the request is refused with 400. |  |
**bucket** | Option<**String**> | Width of each point of the timeline. Defaults to DAY. |  |[default to DAY]
**skill_name** | Option<**String**> | Only count the activity attributed to this skill. Lowercase letters, digits, `.`, `_` or `-`, at most 64 characters, otherwise the request is refused with 400. |  |
**outcome** | Option<[**AgentActionOutcome**](AgentActionOutcome.md)> | DENIED lists refusals only, and leaves skill loads out. ALLOWED lists everything else, skill loads included. |  |
**source** | Option<[**AgentActivitySource**](AgentActivitySource.md)> | Only count the activity from this source. |  |

### Return type

[**models::AgentActivityResponse**](AgentActivityResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

