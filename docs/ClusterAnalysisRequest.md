# ClusterAnalysisRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [**models::ClusterAnalysisKind**](ClusterAnalysisKind.md) |  | 
**output_format** | [**models::ClusterAnalysisOutputFormat**](ClusterAnalysisOutputFormat.md) |  | 
**prometheus_url** | Option<**String**> | Optional Prometheus URL for COST_RECOMMENDATION analysis. When omitted, the engine resolves the default Qovery OBS endpoint. | [optional]
**cmd_args** | Option<**Vec<String>**> | Optional allowlisted KRR arguments for COST_RECOMMENDATION analysis. The engine validates and rejects unsupported or unsafe KRR flags. | [optional]
**target_kubernetes_version** | Option<**String**> | Optional target Kubernetes version for DEPRECATED_API_CHECK analysis. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


