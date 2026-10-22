# LlmProviderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | LLM provider name, unique per scope owner within the organization | 
**description** | Option<**String**> |  | [optional][default to ]
**r#type** | [**models::LlmProviderType**](LlmProviderType.md) |  | 
**credential** | Option<**String**> | The provider credential. Encrypted at rest and never returned by the API. Blank means unchanged: on create no credential is stored, on edit the stored one is kept. Sending a nonblank value rotates it. CLAUDE sends it to Anthropic as an API key. ANTHROPIC_COMPATIBLE sends it to its gateway as a bearer token only, never as an API key. | [optional][default to ]
**scope** | Option<[**models::LlmProviderScope**](LlmProviderScope.md)> | Cannot be changed after creation. On create, omitting it means ORGANIZATION, which requires the MANAGE_INFRASTRUCTURE permission; creating a USER provider requires CREATE_PROJECT. On edit, omitting it leaves the provider's scope unchanged, and stating a scope that differs from the provider's is refused with 400. | [optional]
**region** | Option<**String**> | On edit, omitting it or sending null clears the stored region. This differs from credential, where a blank value keeps the stored one. AWS region the Bedrock client calls, for example us-east-1 or eu-west-1; model availability differs by region. Only allowed for a BEDROCK provider. Any sent string, blank included, must match the pattern. Null keeps the engine default region. | [optional]
**base_url** | Option<**String**> | Root of the custom Anthropic-compatible API. Required for an ANTHROPIC_COMPATIBLE provider and refused with 400 for CLAUDE and BEDROCK. It is an absolute HTTP or HTTPS URL with a host and an optional path prefix, and no user info, query or fragment. A port, when given, must be between 1 and 65535. Like region, an edit replaces it: omitting it or sending null is refused for ANTHROPIC_COMPATIBLE, so an edit must resend it. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


