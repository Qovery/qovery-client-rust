# SkillPresetSecurityReview

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**verdict** | [**models::SkillPresetSecurityReviewVerdict**](SkillPresetSecurityReviewVerdict.md) |  | 
**model** | **String** | The model that reviewed the content. | 
**prompt_version** | **String** | Version of the review instructions the model used. | 
**findings** | [**Vec<models::SkillPresetRejectionFinding>**](SkillPresetRejectionFinding.md) | Notes the model raised without blocking. Usually empty. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


