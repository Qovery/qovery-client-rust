# SkillPresetRevisionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**skill_preset_id** | **uuid::Uuid** | The preset's id. Still listed after the preset is deleted. | 
**revision** | **i32** |  | 
**change_type** | [**models::SkillPresetChangeType**](SkillPresetChangeType.md) |  | 
**edited_by** | **String** | Who saved this state. | 
**edited_at** | **String** |  | 
**name** | **String** |  | 
**description** | **String** |  | 
**base** | [**models::SkillPresetBase**](SkillPresetBase.md) |  | 
**is_template** | **bool** | Whether the preset was a template at this revision. | 
**team_context** | Option<**String**> | Null when the preset had none at this revision. | [optional]
**skills** | [**Vec<models::SkillPresetSkillResponse>**](SkillPresetSkillResponse.md) |  | 
**assigned_to** | [**models::SkillPresetAssignedTo**](SkillPresetAssignedTo.md) |  | 
**security_review** | Option<[**models::SkillPresetSecurityReview**](SkillPresetSecurityReview.md)> | The model review this state passed. Null for a DELETE revision, and for a save made while the review was turned off, when presets are saved after the rules only. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


