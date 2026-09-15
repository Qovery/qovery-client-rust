# PlatformTemplateComponentResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** |  | 
**kind** | [**models::PlatformTemplateComponentKind**](PlatformTemplateComponentKind.md) |  | 
**description** | Option<**String**> |  | [optional]
**fields** | [**Vec<models::FieldSchemaResponse>**](FieldSchemaResponse.md) |  | 
**configuration_sections** | Option<[**Vec<models::PlatformConfigurationSectionResponse>**](PlatformConfigurationSectionResponse.md)> | Additional configuration sections displayed under this component. Its own configuration and cluster inputs remain available. Omitted or empty keeps the existing editor behavior; fields always remain declared on their owner. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


