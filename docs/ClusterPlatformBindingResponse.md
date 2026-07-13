# ClusterPlatformBindingResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** |  | 
**organization_id** | **uuid::Uuid** |  | 
**template_key** | **String** |  | 
**template_version** | **String** |  | 
**layer_selections** | **std::collections::HashMap<String, bool>** |  | 
**managed_config** | **std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>** | Component configuration values keyed by component key | 
**customer_provided_inputs** | **std::collections::HashMap<String, std::collections::HashMap<String, String>>** | String values keyed first by component key and then by input key | 
**layers** | [**Vec<models::ClusterPlatformBindingLayerResponse>**](ClusterPlatformBindingLayerResponse.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


