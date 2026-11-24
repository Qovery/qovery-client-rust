# SkillUsageTotals

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**loads** | **i64** | Skill loads in the range, cached ones included. | 
**cached_loads** | **i64** | Loads answered with a 304, where the agent reused its cached copy. | 
**members** | **i64** | Distinct members whose agents loaded at least one skill. | 
**approx_injected_tokens** | **i64** | Estimated tokens of organization content and team context sent to agents, the number of characters divided by 4. Cached loads send nothing and add zero. The text of the Qovery skills themselves is not counted. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


