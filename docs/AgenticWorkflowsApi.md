# \AgenticWorkflowsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_agentic_workflow**](AgenticWorkflowsApi.md#create_agentic_workflow) | **POST** /environment/{environmentId}/agenticWorkflow | Create an agentic workflow
[**delete_agentic_workflow**](AgenticWorkflowsApi.md#delete_agentic_workflow) | **DELETE** /agenticWorkflow/{agenticWorkflowId} | Delete an agentic workflow
[**edit_agentic_workflow**](AgenticWorkflowsApi.md#edit_agentic_workflow) | **PUT** /agenticWorkflow/{agenticWorkflowId} | Edit an agentic workflow
[**get_agentic_workflow**](AgenticWorkflowsApi.md#get_agentic_workflow) | **GET** /agenticWorkflow/{agenticWorkflowId} | Get an agentic workflow
[**list_agentic_workflows**](AgenticWorkflowsApi.md#list_agentic_workflows) | **GET** /environment/{environmentId}/agenticWorkflow | List agentic workflows



## create_agentic_workflow

> models::AgenticWorkflowResponse create_agentic_workflow(environment_id, agentic_workflow_request)
Create an agentic workflow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**agentic_workflow_request** | Option<[**AgenticWorkflowRequest**](AgenticWorkflowRequest.md)> |  |  |

### Return type

[**models::AgenticWorkflowResponse**](AgenticWorkflowResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_agentic_workflow

> delete_agentic_workflow(agentic_workflow_id)
Delete an agentic workflow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agentic_workflow_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_agentic_workflow

> models::AgenticWorkflowResponse edit_agentic_workflow(agentic_workflow_id, agentic_workflow_request)
Edit an agentic workflow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agentic_workflow_id** | **uuid::Uuid** |  | [required] |
**agentic_workflow_request** | Option<[**AgenticWorkflowRequest**](AgenticWorkflowRequest.md)> |  |  |

### Return type

[**models::AgenticWorkflowResponse**](AgenticWorkflowResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_agentic_workflow

> models::AgenticWorkflowResponse get_agentic_workflow(agentic_workflow_id)
Get an agentic workflow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agentic_workflow_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::AgenticWorkflowResponse**](AgenticWorkflowResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_agentic_workflows

> models::AgenticWorkflowResponseList list_agentic_workflows(environment_id)
List agentic workflows

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |

### Return type

[**models::AgenticWorkflowResponseList**](AgenticWorkflowResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

