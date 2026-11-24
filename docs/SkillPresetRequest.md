# SkillPresetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Preset name, unique in the organization, 1 to 100 characters | 
**description** | Option<**String**> | What the preset is for, at most 1024 characters. Shown to the members it applies to. | [optional][default to ]
**base** | Option<[**models::SkillPresetBase**](SkillPresetBase.md)> | The Qovery skill set the preset starts from. Defaults to FULL. | [optional][default to Full]
**is_template** | Option<**bool**> | Whether this is a template: a starting point admins copy into new presets. A template is never resolved for a member and cannot have assignments (400 otherwise). Defaults to false. | [optional][default to false]
**team_context** | Option<**String**> | Markdown on how the team uses Qovery, delivered with every skill the agent loads, at most 32 KB. Null or blank means none. | [optional]
**skills** | Option<[**Vec<models::SkillPresetSkillRequest>**](SkillPresetSkillRequest.md)> | Changes on top of the base, at most 50, each skill name at most once. | [optional][default to []]
**assigned_to** | Option<[**models::SkillPresetAssignedTo**](SkillPresetAssignedTo.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


