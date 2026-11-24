# SkillPresetManifest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | 
**name** | **String** |  | 
**base** | [**models::SkillPresetBase**](SkillPresetBase.md) |  | 
**revision** | **i32** |  | 
**updated_at** | **String** |  | 
**team_context** | Option<[**models::SkillPresetContentSummary**](SkillPresetContentSummary.md)> | Size and hash of the team context, which comes with every skill fetched. Null when the preset has none. | [optional]
**skills** | [**Vec<models::SkillPresetManifestSkill>**](SkillPresetManifestSkill.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


