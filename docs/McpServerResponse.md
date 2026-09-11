# McpServerResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | **String** |  | [readonly]
**name** | **String** |  | 
**description** | **String** |  | 
**url** | **String** | HTTPS URL of the remote MCP server | 
**header_names** | **HashSet<String>** | Names of the configured HTTP headers. Header values are never returned. | 
**scope** | [**models::McpServerScope**](McpServerScope.md) |  | 
**owner_user_sub** | Option<**String**> | Identity of the owning member. Null for an ORGANIZATION connector. | [optional]
**owner_name** | Option<**String**> | Display name of the owning member. Null for an ORGANIZATION connector. | [optional]
**attachable** | **bool** | Whether the member making this request may attach the connector to an agentic workflow. Computed per caller: an organization admin sees every USER connector but can attach none of them, so a picker must use this rather than scope alone. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


