# LlmProviderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | LLM provider name, unique per scope owner within the organization | 
**description** | Option<**String**> |  | [optional][default to ]
**r#type** | [**models::LlmProviderType**](LlmProviderType.md) |  | 
**credential** | Option<**String**> | The provider credential. Encrypted at rest and never returned by the API. Blank means unchanged: on create no credential is stored, on edit the stored one is kept. Sending a nonblank value rotates it. | [optional][default to ]
**scope** | Option<[**models::LlmProviderScope**](LlmProviderScope.md)> | Cannot be changed after creation. On create, omitting it means ORGANIZATION, which requires the MANAGE_INFRASTRUCTURE permission; creating a USER provider requires CREATE_PROJECT. On edit, omitting it leaves the provider's scope unchanged, and stating a scope that differs from the provider's is refused with 400. | [optional]
**region** | Option<**String**> | On edit, omitting it or sending null clears the stored region. This differs from credential, where a blank value keeps the stored one. AWS region the Bedrock client calls, for example us-east-1 or eu-west-1; model availability differs by region. Only allowed for a BEDROCK provider. Any sent string, blank included, must match the pattern. Null keeps the engine default region. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


