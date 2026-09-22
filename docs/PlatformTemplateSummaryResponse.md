# PlatformTemplateSummaryResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** |  | 
**version** | **String** |  | 
**status** | [**models::PlatformTemplateReleaseStatus**](PlatformTemplateReleaseStatus.md) |  | 
**description** | Option<**String**> |  | [optional]
**bootstrap_component** | Option<[**models::PlatformTemplateComponentResponse**](PlatformTemplateComponentResponse.md)> | Mandatory bootstrap component and its configurable fields, when declared by the template. | [optional]
**layers** | [**Vec<models::PlatformTemplateLayerResponse>**](PlatformTemplateLayerResponse.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


