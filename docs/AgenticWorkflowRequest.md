# AgenticWorkflowRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | name is case insensitive | 
**description** | Option<**String**> |  | [optional][default to ]
**whitelist_hosts** | Option<**Vec<String>**> | Hosts the agentic workflow is allowed to reach | [optional][default to []]
**model_settings** | Option<**String**> |  | [optional][default to ]
**docker_fragment** | Option<**String**> |  | [optional][default to ]
**enabled** | Option<**bool**> |  | [optional][default to true]
**mcp_connectors** | Option<[**Vec<models::AgenticWorkflowConnector>**](AgenticWorkflowConnector.md)> |  | [optional][default to []]
**outputs** | Option<[**Vec<models::AgenticWorkflowOutput>**](AgenticWorkflowOutput.md)> |  | [optional][default to []]
**model** | Option<[**models::AgenticWorkflowModel**](AgenticWorkflowModel.md)> |  | [optional][default to Claude]
**project_repositories** | Option<[**Vec<models::AgenticWorkflowProjectRepository>**](AgenticWorkflowProjectRepository.md)> |  | [optional][default to []]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


