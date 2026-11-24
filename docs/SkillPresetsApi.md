# \SkillPresetsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_skill_preset**](SkillPresetsApi.md#create_skill_preset) | **POST** /organization/{organizationId}/skillPreset | Create a skill preset
[**delete_skill_preset**](SkillPresetsApi.md#delete_skill_preset) | **DELETE** /skillPreset/{skillPresetId} | Delete a skill preset
[**edit_skill_preset**](SkillPresetsApi.md#edit_skill_preset) | **PUT** /skillPreset/{skillPresetId} | Edit a skill preset
[**get_resolved_skill**](SkillPresetsApi.md#get_resolved_skill) | **GET** /organization/{organizationId}/skillPreset/resolved/skill/{skillName} | Get what to load for one skill
[**get_resolved_skill_preset**](SkillPresetsApi.md#get_resolved_skill_preset) | **GET** /organization/{organizationId}/skillPreset/resolved | Get the skill preset that applies to the caller
[**get_skill_preset**](SkillPresetsApi.md#get_skill_preset) | **GET** /skillPreset/{skillPresetId} | Get a skill preset
[**get_skill_usage**](SkillPresetsApi.md#get_skill_usage) | **GET** /organization/{organizationId}/skillPreset/usage | Get skill usage
[**list_skill_preset_revisions**](SkillPresetsApi.md#list_skill_preset_revisions) | **GET** /organization/{organizationId}/skillPreset/revision | List skill preset revisions
[**list_skill_presets**](SkillPresetsApi.md#list_skill_presets) | **GET** /organization/{organizationId}/skillPreset | List organization skill presets



## create_skill_preset

> models::SkillPresetResponse create_skill_preset(organization_id, skill_preset_request)
Create a skill preset

Create a skill preset and its assignments. Requires the MANAGE_ORGANIZATION_DATA permission (organization owner or admin). The preset starts at revision 1.  Set `is_template` to save a template: a starting point admins copy into new presets. A template is never resolved for a member and cannot have assignments.  A preset runs inside each assigned member's coding agent, on their machine, so every create and edit is checked before anything reaches members. The check covers every field that reaches an agent (the preset description, the team context, and each skill's description and content) and runs in two steps:  1. Deterministic rules look for content that could turn the agent against its user: overriding its instructions, hiding actions from the user, skipping confirmations, reading local credentials, dumping secrets, sending data out, running remote or obfuscated code, links outside an allowlist of documentation hosts, invisible characters, HTML comments, encoded blobs and pasted credentials. Any finding answers 422 (findings with `source` RULES) and the model is not called. 2. A model review on AWS Bedrock then reads the content as untrusted data, to catch paraphrased or new attacks the rules miss. A BLOCK verdict answers 422 (findings with `source` MODEL).  The check fails closed: if the model review cannot give a verdict (error, timeout or unusable answer), the request answers 503 and nothing is saved. Because every save is sent to the model, an organization with AI features turned off cannot create or edit presets (403). Reading, resolving, loading and deleting presets are not affected. A save refused for security is recorded in the audit log as a `SKILL_PRESET_REJECTED` event with sub-target `SKILL_PRESET`, holding hashes and findings, never the text.  The creation is recorded in the audit log as a `CREATE` event with sub-target `SKILL_PRESET`, holding the saved state with its full text, and as revision 1 in the revision history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**skill_preset_request** | [**SkillPresetRequest**](SkillPresetRequest.md) |  | [required] |

### Return type

[**models::SkillPresetResponse**](SkillPresetResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_skill_preset

> delete_skill_preset(skill_preset_id)
Delete a skill preset

Delete a skill preset and its assignments. Members it applied to fall back to the next preset in the resolution order, or to the Qovery skills as published. Requires the MANAGE_ORGANIZATION_DATA permission on the preset's organization. The deletion is recorded in the audit log as a `DELETE` event with sub-target `SKILL_PRESET`, holding the deleted state, and as a final `DELETE` revision in the revision history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_preset_id** | **uuid::Uuid** | Skill Preset ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_skill_preset

> models::SkillPresetResponse edit_skill_preset(skill_preset_id, skill_preset_request)
Edit a skill preset

Replace a skill preset, its skill entries and its assignments with the request body, and increase its revision by one. Requires the MANAGE_ORGANIZATION_DATA permission on the preset's organization. The content is checked as on creation, by the rules and then by the model review on AWS Bedrock, and the check fails closed: a finding answers 422, an unavailable review answers 503, and in both cases nothing is saved. An organization with AI features turned off cannot edit presets (403). A refusal for security is recorded as a `SKILL_PRESET_REJECTED` audit event.  The edit is recorded in the audit log as an `UPDATE` event with sub-target `SKILL_PRESET`: the change holds the new state with its full text and a summary of what changed (skills added, removed and modified, and whether the base, the template flag, the team context or the assignments changed), and the original change holds the previous state. When the text of a preset is larger than 64 KB, the audit entry keeps only the SHA-256 of each text and sets `content_omitted`; the revision history always holds the full text.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_preset_id** | **uuid::Uuid** | Skill Preset ID | [required] |
**skill_preset_request** | [**SkillPresetRequest**](SkillPresetRequest.md) |  | [required] |

### Return type

[**models::SkillPresetResponse**](SkillPresetResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resolved_skill

> models::SkillPresetResolvedSkillResponse get_resolved_skill(organization_id, skill_name, purpose, if_none_match)
Get what to load for one skill

Return what the caller's agent must load for one skill, under the preset that applies to the caller (resolved as in `getResolvedSkillPreset`): the organization's content for that skill if any, the team context, and the SHA-256 of each, so the member can check that what their agent follows is what an admin saved. Any organization member can call it (READ_ORGANIZATION_DATA).  A Qovery skill (`qovery` or `qovery-*`) that the preset leaves alone, or any Qovery skill when no preset applies, answers 200 with a null `mode`: load the Qovery skill as published, with the team context if there is one. A `DISABLE` mode means the agent must not load the skill. Any other name that is not in the preset answers 404.  The ETag is the same as the manifest's, so a single revision covers every skill of the preset. Send it in `If-None-Match` to get a 304 with no body when the cached copy is current.  Set `purpose` to say who reads the skill. LOAD (the default) is a member's coding agent loading it to follow it. VIEW is a person reading it, for example in the Console: same body and same ETag and 304 behaviour, recorded as a `SKILL_VIEW` audit event with the same content as a `SKILL_LOAD` entry, and not counted in `getSkillUsage`.  Every LOAD call is recorded in the audit log as a `SKILL_LOAD` event with sub-target `SKILL_PRESET`, 304s included (with `from_cache` set to true): the entry gives the skill, the preset and revision that applied (or `QOVERY_DEFAULT`), the mode, and the SHA-256 of the content and team context that were injected. A 404 is not recorded.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**skill_name** | **String** | Name of the skill to load, for example `qovery-deploy` or a skill the preset adds | [required] |
**purpose** | Option<**String**> | Who reads the skill. Defaults to LOAD. Any other value than LOAD or VIEW is refused with 400. |  |[default to LOAD]
**if_none_match** | Option<**String**> | The ETag of the caller's cached copy. When it matches the current ETag, the response is a 304 with no body. |  |

### Return type

[**models::SkillPresetResolvedSkillResponse**](SkillPresetResolvedSkillResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resolved_skill_preset

> models::SkillPresetManifestResponse get_resolved_skill_preset(organization_id, if_none_match)
Get the skill preset that applies to the caller

Return the manifest of the skill preset that applies to the caller in this organization: its name, base, revision and, for each skill entry, the mode, the description and the size and SHA-256 of the content, but never the content itself. This is what the Qovery loader skill reads first. It then fetches only the skills a task needs with `getResolvedSkill`. Any organization member can call it (READ_ORGANIZATION_DATA). Assignments and the preset's creator are left out.  The preset is resolved in this order: a preset assigned to the caller as a member, then a preset assigned to the caller's role in the organization, then the organization default. When none applies, `source` is `QOVERY_DEFAULT`, `skill_preset` is null and the agent uses the Qovery skills as published. API tokens have no member behind them, so they only receive a role preset or the organization default.  The response carries an `ETag`. Treat it as opaque: it changes when the preset is edited, when the caller moves to another preset, and when the caller reaches the same preset through another assignment. Send it back in `If-None-Match` to get a 304 with no body when nothing changed. Templates are never resolved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**if_none_match** | Option<**String**> | The ETag of the caller's cached copy. When it matches the current ETag, the response is a 304 with no body. |  |

### Return type

[**models::SkillPresetManifestResponse**](SkillPresetManifestResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_skill_preset

> models::SkillPresetResponse get_skill_preset(skill_preset_id)
Get a skill preset

Get a skill preset with its full content and assignments. Requires the MANAGE_ORGANIZATION_DATA permission on the preset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_preset_id** | **uuid::Uuid** | Skill Preset ID | [required] |

### Return type

[**models::SkillPresetResponse**](SkillPresetResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_skill_usage

> models::SkillUsageResponse get_skill_usage(organization_id, from, to, bucket, skill_name, user_sub, skill_preset_id)
Get skill usage

Return how the members' coding agents loaded skills in the organization over a time range: totals, a timeline, and breakdowns by skill, by member and by preset. Requires the MANAGE_ORGANIZATION_DATA permission (organization owner or admin).  Usage counts agent loads only: a load is one call to `getResolvedSkill` with `purpose` LOAD (the default) that returned a skill, cached ones included. A 304 counts as a cached load, a 404 is not counted, and VIEW calls (a person reading a skill) are never counted. Loads are counted whether or not a preset applies. Recording a load is best effort and never blocks the agent, so a load can be missing here while the audit log, which records every load as a `SKILL_LOAD` event, still has it.  The range defaults to the 30 days ending now and spans at most 90 days. Timeline buckets are cut in UTC, and buckets with no load are left out rather than returned as zero. Rows in `by_skill` and `by_member` are sorted by loads, most first, and capped at 100 and 500 rows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**from** | Option<**String**> | Start of the range, inclusive. Defaults to 30 days before `to`. |  |
**to** | Option<**String**> | End of the range, exclusive. Defaults to now. Must be after `from`, and the range must not exceed 90 days, otherwise the request is refused with 400. |  |
**bucket** | Option<**String**> | Width of each point of the timeline. Defaults to DAY. |  |[default to DAY]
**skill_name** | Option<**String**> | Only count the loads of this skill. |  |
**user_sub** | Option<**String**> | Only count the loads of this member, by the id returned by \"List organization members\". |  |
**skill_preset_id** | Option<**uuid::Uuid**> | Only count the loads made under this preset. |  |

### Return type

[**models::SkillUsageResponse**](SkillUsageResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_skill_preset_revisions

> models::SkillPresetRevisionResponseList list_skill_preset_revisions(organization_id, skill_preset_id, limit)
List skill preset revisions

List the saved states of the organization's skill presets, most recent first, each with its full content, who saved it and when. Every create, edit and delete adds one immutable revision. Revisions are kept after a preset is deleted: the deletion adds a final `DELETE` revision holding the state the preset had. A save that fails, for example with a 422, adds nothing. Requires the MANAGE_ORGANIZATION_DATA permission (organization owner or admin).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**skill_preset_id** | Option<**uuid::Uuid**> | Only list the revisions of this preset, deleted or not. Omit it to list the revisions of every preset of the organization. |  |
**limit** | Option<**i32**> | Maximum number of revisions to return. A value outside 1 to 100 is brought back into that range. |  |[default to 50]

### Return type

[**models::SkillPresetRevisionResponseList**](SkillPresetRevisionResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_skill_presets

> models::SkillPresetResponseList list_skill_presets(organization_id, is_template)
List organization skill presets

List the skill presets of the organization, with their full content and who each one is assigned to. Templates are listed too unless `isTemplate` filters them. Requires the MANAGE_ORGANIZATION_DATA permission (organization owner or admin).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**is_template** | Option<**bool**> | `true` lists templates only, `false` lists assignable presets only. Omit it to list both. |  |

### Return type

[**models::SkillPresetResponseList**](SkillPresetResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

