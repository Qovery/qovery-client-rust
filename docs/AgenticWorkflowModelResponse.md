# AgenticWorkflowModelResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | [**models::AgenticWorkflowModelType**](AgenticWorkflowModelType.md) |  | 
**settings** | **String** |  | 
**llm_provider_id** | Option<**uuid::Uuid**> | The LLM provider the workflow takes its credential from, or null when it carries its own `api_key`. Unlike `api_key` this is returned: it names a credential rather than carrying one. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


