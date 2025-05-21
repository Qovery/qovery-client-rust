# \HelmDeploymentRestrictionApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_helm_deployment_restriction**](HelmDeploymentRestrictionApi.md#create_helm_deployment_restriction) | **POST** /helm/{helmId}/deploymentRestriction | Create a helm deployment restriction
[**delete_helm_deployment_restriction**](HelmDeploymentRestrictionApi.md#delete_helm_deployment_restriction) | **DELETE** /helm/{helmId}/deploymentRestriction/{deploymentRestrictionId} | Delete a helm deployment restriction
[**edit_helm_deployment_restriction**](HelmDeploymentRestrictionApi.md#edit_helm_deployment_restriction) | **PUT** /helm/{helmId}/deploymentRestriction/{deploymentRestrictionId} | Edit a helm deployment restriction
[**get_helm_deployment_restrictions**](HelmDeploymentRestrictionApi.md#get_helm_deployment_restrictions) | **GET** /helm/{helmId}/deploymentRestriction | Get helm deployment restrictions



## create_helm_deployment_restriction

> models::HelmDeploymentRestrictionResponse create_helm_deployment_restriction(helm_id, helm_deployment_restriction_request)
Create a helm deployment restriction

Create a helm deployment restriction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**helm_id** | **uuid::Uuid** | Helm ID | [required] |
**helm_deployment_restriction_request** | Option<[**HelmDeploymentRestrictionRequest**](HelmDeploymentRestrictionRequest.md)> |  |  |

### Return type

[**models::HelmDeploymentRestrictionResponse**](HelmDeploymentRestrictionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_helm_deployment_restriction

> delete_helm_deployment_restriction(helm_id, deployment_restriction_id)
Delete a helm deployment restriction

Delete a helm deployment restriction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**helm_id** | **uuid::Uuid** | Helm ID | [required] |
**deployment_restriction_id** | **uuid::Uuid** | Deployment Restriction ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_helm_deployment_restriction

> models::HelmDeploymentRestrictionResponse edit_helm_deployment_restriction(helm_id, deployment_restriction_id, helm_deployment_restriction_request)
Edit a helm deployment restriction

Edit a helm deployment restriction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**helm_id** | **uuid::Uuid** | Helm ID | [required] |
**deployment_restriction_id** | **uuid::Uuid** | Deployment Restriction ID | [required] |
**helm_deployment_restriction_request** | Option<[**HelmDeploymentRestrictionRequest**](HelmDeploymentRestrictionRequest.md)> |  |  |

### Return type

[**models::HelmDeploymentRestrictionResponse**](HelmDeploymentRestrictionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_helm_deployment_restrictions

> models::HelmDeploymentRestrictionResponseList get_helm_deployment_restrictions(helm_id)
Get helm deployment restrictions

Get helm deployment restrictions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**helm_id** | **uuid::Uuid** | Helm ID | [required] |

### Return type

[**models::HelmDeploymentRestrictionResponseList**](HelmDeploymentRestrictionResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

