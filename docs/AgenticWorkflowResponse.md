# AgenticWorkflowResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | 
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | **String** | name is case insensitive | 
**description** | **String** |  | 
**whitelist_hosts** | **Vec<String>** | Hosts the agentic workflow is allowed to reach | 
**model_settings** | **String** |  | 
**docker_fragment** | **String** |  | 
**enabled** | **bool** |  | 
**mcp_connectors** | [**Vec<models::AgenticWorkflowConnector>**](AgenticWorkflowConnector.md) |  | 
**outputs** | [**Vec<models::AgenticWorkflowOutput>**](AgenticWorkflowOutput.md) |  | 
**model** | [**models::AgenticWorkflowModel**](AgenticWorkflowModel.md) |  | 
**project_repositories** | [**Vec<models::AgenticWorkflowProjectRepository>**](AgenticWorkflowProjectRepository.md) |  | 
**webhook** | [**models::AgenticWorkflowWebhook**](AgenticWorkflowWebhook.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


