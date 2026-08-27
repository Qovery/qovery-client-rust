# AgenticWorkflowResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | 
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**service_type** | [**models::ServiceTypeEnum**](ServiceTypeEnum.md) |  | 
**environment** | [**models::ReferenceObject**](ReferenceObject.md) |  | 
**name** | **String** | name is case insensitive | 
**slug** | **String** | URL-friendly identifier derived from the name | 
**description** | **String** |  | 
**webhook_ip_allowlist** | **Vec<String>** | CIDR ranges the incoming webhook request's source IP is checked against | 
**docker_fragment** | **String** |  | 
**enabled** | **bool** |  | 
**mcp** | **String** | Raw JSON blob describing the MCP servers configured for this workflow | 
**mcp_server_ids** | **Vec<uuid::Uuid>** | Organization MCP servers used by this workflow | 
**outputs** | [**Vec<models::AgenticWorkflowOutput>**](AgenticWorkflowOutput.md) |  | 
**model** | [**models::AgenticWorkflowModelResponse**](AgenticWorkflowModelResponse.md) |  | 
**project_repositories** | [**Vec<models::AgenticWorkflowProjectRepository>**](AgenticWorkflowProjectRepository.md) |  | 
**agent_prompt** | **String** |  | 
**governance** | [**models::AgenticWorkflowGovernance**](AgenticWorkflowGovernance.md) |  | 
**resources** | [**models::AgenticWorkflowResources**](AgenticWorkflowResources.md) |  | 
**webhook** | [**models::AgenticWorkflowWebhook**](AgenticWorkflowWebhook.md) |  | 
**schedule** | Option<[**models::AgenticWorkflowScheduleResponse**](AgenticWorkflowScheduleResponse.md)> | Cron schedule firing runs of this workflow. Null when the workflow is webhook-only. | 
**icon_uri** | Option<**String**> | Icon URI representing the agentic workflow. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


