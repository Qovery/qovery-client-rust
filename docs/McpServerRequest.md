# McpServerRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Unique MCP server name within the organization | 
**description** | Option<**String**> |  | [optional][default to ]
**url** | **String** | HTTPS URL of the remote MCP server | 
**headers** | Option<**std::collections::HashMap<String, String>**> | HTTP headers sent to the MCP server. Header values are encrypted and never returned by the API. | [optional][default to {}]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


