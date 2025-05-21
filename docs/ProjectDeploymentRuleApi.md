# \ProjectDeploymentRuleApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_deployment_rule**](ProjectDeploymentRuleApi.md#create_deployment_rule) | **POST** /project/{projectId}/deploymentRule | Create a deployment rule
[**delete_project_deployment_rule**](ProjectDeploymentRuleApi.md#delete_project_deployment_rule) | **DELETE** /project/{projectId}/deploymentRule/{deploymentRuleId} | Delete a project deployment rule
[**edit_project_deployemtn_rule**](ProjectDeploymentRuleApi.md#edit_project_deployemtn_rule) | **PUT** /project/{projectId}/deploymentRule/{deploymentRuleId} | Edit a project deployment rule
[**get_project_deployment_rule**](ProjectDeploymentRuleApi.md#get_project_deployment_rule) | **GET** /project/{projectId}/deploymentRule/{deploymentRuleId} | Get a project deployment rule
[**list_project_deployment_rules**](ProjectDeploymentRuleApi.md#list_project_deployment_rules) | **GET** /project/{projectId}/deploymentRule | List project deployment rules
[**update_deployment_rules_priority_order**](ProjectDeploymentRuleApi.md#update_deployment_rules_priority_order) | **PUT** /project/{projectId}/deploymentRule/order | Update deployment rules priority order



## create_deployment_rule

> models::ProjectDeploymentRule create_deployment_rule(project_id, project_deployment_rule_request)
Create a deployment rule

Create a deployment rule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |
**project_deployment_rule_request** | Option<[**ProjectDeploymentRuleRequest**](ProjectDeploymentRuleRequest.md)> |  |  |

### Return type

[**models::ProjectDeploymentRule**](ProjectDeploymentRule.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project_deployment_rule

> delete_project_deployment_rule(project_id, deployment_rule_id)
Delete a project deployment rule

Delete a project deployment rule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |
**deployment_rule_id** | **uuid::Uuid** | Deployment Rule ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_project_deployemtn_rule

> models::ProjectDeploymentRule edit_project_deployemtn_rule(project_id, deployment_rule_id, project_deployment_rule_request)
Edit a project deployment rule

Edit a project deployment rule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |
**deployment_rule_id** | **uuid::Uuid** | Deployment Rule ID | [required] |
**project_deployment_rule_request** | Option<[**ProjectDeploymentRuleRequest**](ProjectDeploymentRuleRequest.md)> |  |  |

### Return type

[**models::ProjectDeploymentRule**](ProjectDeploymentRule.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_deployment_rule

> models::ProjectDeploymentRule get_project_deployment_rule(project_id, deployment_rule_id)
Get a project deployment rule

Get a project deployment rule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |
**deployment_rule_id** | **uuid::Uuid** | Deployment Rule ID | [required] |

### Return type

[**models::ProjectDeploymentRule**](ProjectDeploymentRule.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_project_deployment_rules

> models::ProjectDeploymentRuleResponseList list_project_deployment_rules(project_id)
List project deployment rules

List project deployment rules

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |

### Return type

[**models::ProjectDeploymentRuleResponseList**](ProjectDeploymentRuleResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_deployment_rules_priority_order

> update_deployment_rules_priority_order(project_id, project_deployment_rules_priority_order_request)
Update deployment rules priority order

Update deployment rules priority order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID | [required] |
**project_deployment_rules_priority_order_request** | Option<[**ProjectDeploymentRulesPriorityOrderRequest**](ProjectDeploymentRulesPriorityOrderRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

