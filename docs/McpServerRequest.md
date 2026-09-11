# McpServerRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | MCP server name, unique per scope owner within the organization | 
**description** | Option<**String**> |  | [optional][default to ]
**url** | **String** | HTTPS URL of the remote MCP server | 
**headers** | Option<**std::collections::HashMap<String, String>**> | HTTP headers sent to the MCP server. Header values are encrypted and never returned by the API. | [optional][default to {}]
**scope** | Option<[**models::McpServerScope**](McpServerScope.md)> | Cannot be changed after creation. On create, omitting it means ORGANIZATION, which requires the MANAGE_INFRASTRUCTURE permission; creating a USER connector requires CREATE_PROJECT. On edit, omitting it leaves the connector's scope unchanged, and stating a scope that differs from the connector's is refused with 400. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


