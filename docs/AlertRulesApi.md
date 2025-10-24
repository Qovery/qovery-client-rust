# \AlertRulesApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_alert_rule**](AlertRulesApi.md#create_alert_rule) | **POST** /api/alert-rules | Create alert rule
[**delete_alert_rule**](AlertRulesApi.md#delete_alert_rule) | **DELETE** /api/alert-rules/{alertRuleId} | Delete alert rule
[**edit_alert_rule**](AlertRulesApi.md#edit_alert_rule) | **PUT** /api/alert-rules/{alertRuleId} | Update alert rule
[**get_alert_rule**](AlertRulesApi.md#get_alert_rule) | **GET** /api/alert-rules/{alertRuleId} | Get alert rule
[**get_alert_rules**](AlertRulesApi.md#get_alert_rules) | **GET** /organization/{organizationId}/alert-rules | List alert rules



## create_alert_rule

> models::AlertRuleResponse create_alert_rule(alert_rule_creation_request)
Create alert rule

Create a new alert rule with PromQL expression

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_rule_creation_request** | Option<[**AlertRuleCreationRequest**](AlertRuleCreationRequest.md)> |  |  |

### Return type

[**models::AlertRuleResponse**](AlertRuleResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_alert_rule

> delete_alert_rule(alert_rule_id)
Delete alert rule

Delete an alert rule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_rule_id** | **uuid::Uuid** | Alert Rule ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_alert_rule

> models::AlertRuleResponse edit_alert_rule(alert_rule_id, alert_rule_edit_request)
Update alert rule

Update an existing alert rule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_rule_id** | **uuid::Uuid** | Alert Rule ID | [required] |
**alert_rule_edit_request** | Option<[**AlertRuleEditRequest**](AlertRuleEditRequest.md)> |  |  |

### Return type

[**models::AlertRuleResponse**](AlertRuleResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alert_rule

> models::AlertRuleResponse get_alert_rule(alert_rule_id)
Get alert rule

Retrieve a specific alert rule by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_rule_id** | **uuid::Uuid** | Alert Rule ID | [required] |

### Return type

[**models::AlertRuleResponse**](AlertRuleResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alert_rules

> models::AlertRuleList get_alert_rules(organization_id)
List alert rules

Retrieve all alert rules for a specific organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::AlertRuleList**](AlertRuleList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

