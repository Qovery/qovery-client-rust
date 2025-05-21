# \EnvironmentDeploymentRuleApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**edit_environment_deployment_rule**](EnvironmentDeploymentRuleApi.md#edit_environment_deployment_rule) | **PUT** /environment/{environmentId}/deploymentRule/{deploymentRuleId} | Edit an environment deployment rule
[**get_environment_deployment_rule**](EnvironmentDeploymentRuleApi.md#get_environment_deployment_rule) | **GET** /environment/{environmentId}/deploymentRule | Get environment deployment rule



## edit_environment_deployment_rule

> models::EnvironmentDeploymentRule edit_environment_deployment_rule(environment_id, deployment_rule_id, environment_deployment_rule_edit_request)
Edit an environment deployment rule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**deployment_rule_id** | **uuid::Uuid** | Deployment Rule ID | [required] |
**environment_deployment_rule_edit_request** | Option<[**EnvironmentDeploymentRuleEditRequest**](EnvironmentDeploymentRuleEditRequest.md)> |  |  |

### Return type

[**models::EnvironmentDeploymentRule**](EnvironmentDeploymentRule.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_environment_deployment_rule

> models::EnvironmentDeploymentRule get_environment_deployment_rule(environment_id)
Get environment deployment rule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |

### Return type

[**models::EnvironmentDeploymentRule**](EnvironmentDeploymentRule.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

