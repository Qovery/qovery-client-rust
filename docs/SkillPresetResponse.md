# SkillPresetResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | **String** |  | [readonly]
**name** | **String** |  | 
**description** | **String** |  | 
**base** | [**models::SkillPresetBase**](SkillPresetBase.md) |  | 
**is_template** | **bool** | Whether this is a template, never resolved for a member and never assigned. | 
**team_context** | Option<**String**> | Markdown delivered with every skill the agent loads. Null when the preset has none. | [optional]
**skills** | [**Vec<models::SkillPresetSkillResponse>**](SkillPresetSkillResponse.md) |  | 
**revision** | **i32** | Starts at 1 and increases by one on every edit. | 
**creator_name** | Option<**String**> | Name of the member who created the preset. Null when unknown. | [optional]
**assigned_to** | [**models::SkillPresetAssignedTo**](SkillPresetAssignedTo.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


