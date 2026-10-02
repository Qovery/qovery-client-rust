# PlatformComponentInputRequirementResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** |  | 
**r#type** | **Type** | Field type understood by the Console. (enum: string, number, bool) | 
**required** | **bool** |  | 
**default_value** | Option<**String**> |  | [optional]
**label** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**sensitive** | **bool** |  | 
**read_only** | Option<**bool**> | Omitted when false. A read-only field is never required or sensitive, and its defaultValue is null. Its value comes from resolvedValues in the same response, never from the client draft. Read-only fields appear only at the top level of the fields of a resolve response; catalog descriptors are never read-only. | [optional]
**constraints** | [**models::FieldSchemaConstraintsResponse**](FieldSchemaConstraintsResponse.md) |  | 
**format** | Option<**String**> | Optional editor format for a string field, independent of its scalar type. kubernetes-resource-yaml selects a single Kubernetes YAML object editor. Unknown formats should fall back to the ordinary string editor. | [optional]
**templates** | Option<[**Vec<models::FieldTemplateResponse>**](FieldTemplateResponse.md)> | Optional starting texts for an explicit user choice, with unique IDs within the field. Present only with format. Never apply as defaults or overwrite a saved value. The format selects the editor even when templates are absent. | [optional]
**scope** | [**models::PlatformComponentConfigurationInputScope**](PlatformComponentConfigurationInputScope.md) |  | 
**status** | [**models::PlatformComponentConfigurationRequirementStatus**](PlatformComponentConfigurationRequirementStatus.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


