# BuildAnalysisResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**confidence** | [**models::BuildAnalysisConfidence**](BuildAnalysisConfidence.md) |  | 
**estimated_time_saved** | Option<[**models::BuildAnalysisTimeSaved**](BuildAnalysisTimeSaved.md)> | Build time saved on each build once every improvement is applied. Null when there is nothing to save or no timing to base it on. `confidence` applies to this range. | 
**assets_reviewed** | [**Vec<models::BuildAnalysisAsset>**](BuildAnalysisAsset.md) |  | 
**summary** | **String** | What slows the build and why, with the evidence from the build. | 
**improvements** | **Vec<String>** | Changes to make, one sentence each, ordered by impact on build time, highest first. Empty when the build is already efficient. | 
**dockerfile** | Option<**String**> | Reworked Dockerfile to copy. Null when no improvement is a Dockerfile change, for example a variable value. Values hidden from the analysis appear as `<redacted>` and must be put back before use. | 
**dockerfile_rationale** | Option<**String**> | Why the reworked Dockerfile is faster. | 
**agent_prompt** | **String** | Prompt the customer pastes into their own AI agent to apply the improvements in their repository. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


