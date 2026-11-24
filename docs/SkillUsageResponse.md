# SkillUsageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from** | **String** | Start of the range actually used, after defaults. | 
**to** | **String** | End of the range actually used, after defaults. | 
**bucket** | [**models::SkillUsageBucket**](SkillUsageBucket.md) |  | 
**totals** | [**models::SkillUsageTotals**](SkillUsageTotals.md) |  | 
**timeline** | [**Vec<models::SkillUsagePoint>**](SkillUsagePoint.md) | Loads per hour or per day, oldest first. Buckets are cut in UTC, and buckets with no load are left out rather than returned as zero. | 
**by_skill** | [**Vec<models::SkillUsageBySkill>**](SkillUsageBySkill.md) | Loads per skill, most loaded first, at most 100 rows. | 
**by_member** | [**Vec<models::SkillUsageByMember>**](SkillUsageByMember.md) | Loads per member, most active first, at most 500 rows. | 
**by_preset** | [**Vec<models::SkillUsageByPreset>**](SkillUsageByPreset.md) | Loads per preset, most loaded first. A row with a null preset groups the loads made with no preset. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


