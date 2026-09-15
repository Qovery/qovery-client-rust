# LlmProviderResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | **String** |  | [readonly]
**name** | **String** |  | 
**description** | **String** |  | 
**r#type** | [**models::LlmProviderType**](LlmProviderType.md) |  | 
**has_credential** | **bool** | Whether a credential is stored. The credential itself is never returned. | 
**scope** | [**models::LlmProviderScope**](LlmProviderScope.md) |  | 
**owner_user_sub** | Option<**String**> | Identity of the owning member. Null for an ORGANIZATION provider. | [optional]
**owner_name** | Option<**String**> | Display name of the owning member. Null for an ORGANIZATION provider. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


