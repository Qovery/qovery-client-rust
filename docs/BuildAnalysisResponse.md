# BuildAnalysisResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**service_id** | **uuid::Uuid** |  | 
**execution_id** | **String** | Execution id of the analysed build. | 
**status** | [**models::BuildAnalysisStatus**](BuildAnalysisStatus.md) |  | 
**stale** | **bool** | True when the latest build of the service is not the analysed one. The analysis stays readable, but only the latest build can be analysed. | 
**dockerfile_source** | [**models::BuildAnalysisDockerfileSource**](BuildAnalysisDockerfileSource.md) |  | 
**failure_code** | Option<[**models::BuildAnalysisFailureCode**](BuildAnalysisFailureCode.md)> | Why the analysis failed, when `status` is FAILED. | 
**failure_reason** | Option<**String**> | Human-readable detail of the failure, when `status` is FAILED. | 
**result** | Option<[**models::BuildAnalysisResult**](BuildAnalysisResult.md)> | Set when `status` is DONE. | 
**created_at** | **String** |  | 
**updated_at** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


