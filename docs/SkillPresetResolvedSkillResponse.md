# SkillPresetResolvedSkillResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source** | [**models::SkillPresetResolutionSource**](SkillPresetResolutionSource.md) |  | 
**skill_name** | **String** |  | 
**skill_preset** | Option<[**models::SkillPresetReference**](SkillPresetReference.md)> | The preset that applies to the caller. Null when `source` is QOVERY_DEFAULT. | [optional]
**mode** | Option<[**models::SkillPresetSkillMode**](SkillPresetSkillMode.md)> | Null when the preset leaves this Qovery skill alone, or when no preset applies: load the Qovery skill as published. DISABLE means the agent must not load the skill. | [optional]
**description** | Option<**String**> | The entry's description. Null when the preset leaves the skill alone or the entry has none. | [optional]
**content** | Option<**String**> | Markdown to load: team instructions after the Qovery skill for OVERLAY, the whole skill for REPLACE and ADD. Null for DISABLE and when the preset leaves the skill alone. | [optional]
**content_sha256** | Option<**String**> | Hex-encoded SHA-256 of `content`. Null when `content` is null. | [optional]
**team_context** | Option<**String**> | The preset's team context, delivered with every skill. Null when there is none or no preset applies. | [optional]
**team_context_sha256** | Option<**String**> | Hex-encoded SHA-256 of `team_context`. Null when `team_context` is null. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


