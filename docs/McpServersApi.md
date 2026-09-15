# \McpServersApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_mcp_server**](McpServersApi.md#create_mcp_server) | **POST** /organization/{organizationId}/mcpServer | Create an MCP server
[**create_qovery_mcp_server**](McpServersApi.md#create_qovery_mcp_server) | **POST** /organization/{organizationId}/mcpServer/qovery | Create a read-only Qovery MCP connector
[**delete_mcp_server**](McpServersApi.md#delete_mcp_server) | **DELETE** /mcpServer/{mcpServerId} | Delete an MCP server
[**edit_mcp_server**](McpServersApi.md#edit_mcp_server) | **PUT** /mcpServer/{mcpServerId} | Edit an MCP server
[**get_mcp_server**](McpServersApi.md#get_mcp_server) | **GET** /mcpServer/{mcpServerId} | Get an MCP server
[**list_mcp_servers**](McpServersApi.md#list_mcp_servers) | **GET** /organization/{organizationId}/mcpServer | List organization MCP servers



## create_mcp_server

> models::McpServerResponse create_mcp_server(organization_id, mcp_server_request)
Create an MCP server

Configure a remote HTTPS MCP server for an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**mcp_server_request** | [**McpServerRequest**](McpServerRequest.md) |  | [required] |

### Return type

[**models::McpServerResponse**](McpServerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_qovery_mcp_server

> models::McpServerResponse create_qovery_mcp_server(organization_id, qovery_mcp_server_request)
Create a read-only Qovery MCP connector

Create a connector to Qovery's own MCP server in READ-ONLY mode. Scope is always `ORGANIZATION`; this endpoint accepts no scope parameter.  During the request, an organization API token is generated, bound to the Viewer role, and stored as the encrypted `Authorization` connector header. The token is never returned in the response; `header_names` will contain `Authorization`.  The generated token is visible and revocable in the organization's API token list. Deleting the connector does not revoke the generated token; revoke it separately from the API token list.  **403 — Access forbidden:** Requires the `MANAGE_INFRASTRUCTURE` permission. Viewer and Billing callers cannot mint API tokens and are rejected with 403.  Only one Qovery MCP connector is allowed per organization; a second call returns 409. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**qovery_mcp_server_request** | Option<[**QoveryMcpServerRequest**](QoveryMcpServerRequest.md)> |  |  |

### Return type

[**models::McpServerResponse**](McpServerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_mcp_server

> delete_mcp_server(mcp_server_id)
Delete an MCP server

Delete a remote MCP server configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mcp_server_id** | **uuid::Uuid** | MCP Server ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_mcp_server

> models::McpServerResponse edit_mcp_server(mcp_server_id, mcp_server_request)
Edit an MCP server

Replace a remote MCP server configuration, including all configured headers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mcp_server_id** | **uuid::Uuid** | MCP Server ID | [required] |
**mcp_server_request** | [**McpServerRequest**](McpServerRequest.md) |  | [required] |

### Return type

[**models::McpServerResponse**](McpServerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mcp_server

> models::McpServerResponse get_mcp_server(mcp_server_id)
Get an MCP server

Get a remote MCP server configuration. Header values are never returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mcp_server_id** | **uuid::Uuid** | MCP Server ID | [required] |

### Return type

[**models::McpServerResponse**](McpServerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_mcp_servers

> models::McpServerResponseList list_mcp_servers(organization_id)
List organization MCP servers

List the remote MCP servers configured for an organization. Header values are never returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::McpServerResponseList**](McpServerResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

