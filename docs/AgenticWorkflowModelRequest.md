# AgenticWorkflowModelRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | [**models::AgenticWorkflowModelType**](AgenticWorkflowModelType.md) |  | 
**api_key** | Option<**String**> | Write-only. Provider API key; accepted on create/edit but never returned in responses. | [optional][default to ]
**settings** | Option<**String**> |  | [optional][default to ]
**llm_provider_id** | Option<**uuid::Uuid**> | An existing LLM provider to take the credential from, instead of `api_key`. The two are mutually exclusive: a request setting both is rejected. The provider must belong to the workflow's organization, be one the caller may use, and match `type`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


