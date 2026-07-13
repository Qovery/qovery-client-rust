# ClusterPlatformBindingRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**template_key** | **String** |  | 
**template_version** | **String** |  | 
**layer_selections** | Option<**std::collections::HashMap<String, bool>**> |  | [optional]
**managed_config** | Option<**std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>**> | Component configuration values keyed by component key | [optional]
**customer_provided_inputs** | Option<**std::collections::HashMap<String, std::collections::HashMap<String, String>>**> | String values keyed first by component key and then by input key | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


