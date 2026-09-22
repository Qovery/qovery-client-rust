# PlatformComponentConfigurationPreviewRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**profile_config** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Configuration values keyed by their catalog field name | [optional]
**replace_profile_config** | Option<**bool**> | For an existing cluster, validate profileConfig as a complete draft instead of merging it with saved values. Omitted keys are reset to their catalog defaults. Template previews already use a complete draft and ignore this flag. | [optional][default to false]
**cluster_inputs** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**component_outputs** | Option<**std::collections::HashMap<String, std::collections::HashMap<String, String>>**> | String values keyed first by component key and then by input key | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


