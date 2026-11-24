# SkillPresetSkillRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**skill_name** | **String** | Lowercase letters, digits and hyphens, at most 64 characters. A Qovery skill name (`qovery` or `qovery-*`) for DISABLE, OVERLAY and REPLACE; any other name for ADD. | 
**mode** | [**models::SkillPresetSkillMode**](SkillPresetSkillMode.md) |  | 
**description** | Option<**String**> | When the agent should use the skill, at most 1024 characters. Required for ADD and REPLACE, since agents pick a skill from its description. DISABLE and OVERLAY take none: a non-blank value is refused with 400. | [optional]
**content** | Option<**String**> | Markdown delivered to the agent, at most 64 KB. Required for every mode except DISABLE, which takes none: a non-blank value is refused with 400. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


