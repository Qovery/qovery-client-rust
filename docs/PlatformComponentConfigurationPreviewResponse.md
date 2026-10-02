# PlatformComponentConfigurationPreviewResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** |  | 
**component_key** | **String** |  | 
**fields** | [**Vec<models::FieldSchemaResponse>**](FieldSchemaResponse.md) |  | 
**requirements** | [**Vec<models::PlatformComponentInputRequirementResponse>**](PlatformComponentInputRequirementResponse.md) |  | 
**component_bindings** | [**Vec<models::PlatformComponentOutputBindingResponse>**](PlatformComponentOutputBindingResponse.md) |  | 
**violations** | [**Vec<models::PlatformComponentConfigurationViolationResponse>**](PlatformComponentConfigurationViolationResponse.md) |  | 
**resolved_values** | **std::collections::HashMap<String, String>** | Value of each read-only field in fields, keyed by field key, with no other entry; `{}` when no field is read-only. Values are encoded as strings, like defaultValue, and `null` means that no value is set, such as no CPU limit. A value is the configuration computed for this draft, not proof of what runs on the cluster. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


