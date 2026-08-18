# ClusterOperatorBootstrapResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chart_repository** | **String** |  | 
**chart_name** | **String** |  | 
**chart_version** | **String** |  | 
**chart_reference** | **String** |  | 
**namespace** | **String** |  | 
**release_name** | **String** |  | 
**values** | **std::collections::HashMap<String, serde_json::Value>** | Structured Helm values. This object can contain cluster credentials. | 
**values_yaml** | **String** | Ready-to-write Helm values file. This value can contain cluster credentials. | 
**helm_command** | **String** | Ready-to-run Helm upgrade/install command using values_yaml. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


