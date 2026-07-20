# AgenticWorkflowRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | name is case insensitive | 
**description** | Option<**String**> |  | [optional][default to ]
**webhook_ip_allowlist** | Option<**Vec<String>**> | CIDR ranges the incoming webhook request's source IP is checked against | [optional][default to []]
**docker_fragment** | Option<**String**> |  | [optional][default to ]
**enabled** | Option<**bool**> |  | [optional][default to true]
**mcp** | Option<**String**> | Raw JSON blob describing the MCP servers configured for this workflow | [optional][default to ]
**outputs** | Option<[**Vec<models::AgenticWorkflowOutput>**](AgenticWorkflowOutput.md)> |  | [optional][default to []]
**model** | Option<[**models::AgenticWorkflowModelRequest**](AgenticWorkflowModelRequest.md)> |  | [optional]
**project_repositories** | Option<[**Vec<models::AgenticWorkflowProjectRepository>**](AgenticWorkflowProjectRepository.md)> |  | [optional][default to []]
**agent_prompt** | Option<**String**> |  | [optional][default to ]
**governance** | Option<[**models::AgenticWorkflowGovernance**](AgenticWorkflowGovernance.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


