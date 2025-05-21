# \ApplicationDeploymentRestrictionApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_application_deployment_restriction**](ApplicationDeploymentRestrictionApi.md#create_application_deployment_restriction) | **POST** /application/{applicationId}/deploymentRestriction | Create an application deployment restriction
[**delete_application_deployment_restriction**](ApplicationDeploymentRestrictionApi.md#delete_application_deployment_restriction) | **DELETE** /application/{applicationId}/deploymentRestriction/{deploymentRestrictionId} | Delete an application deployment restriction
[**edit_application_deployment_restriction**](ApplicationDeploymentRestrictionApi.md#edit_application_deployment_restriction) | **PUT** /application/{applicationId}/deploymentRestriction/{deploymentRestrictionId} | Edit an application deployment restriction
[**get_application_deployment_restrictions**](ApplicationDeploymentRestrictionApi.md#get_application_deployment_restrictions) | **GET** /application/{applicationId}/deploymentRestriction | Get application deployment restrictions



## create_application_deployment_restriction

> models::ApplicationDeploymentRestriction create_application_deployment_restriction(application_id, application_deployment_restriction_request)
Create an application deployment restriction

Create an application deployment restriction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |
**application_deployment_restriction_request** | Option<[**ApplicationDeploymentRestrictionRequest**](ApplicationDeploymentRestrictionRequest.md)> |  |  |

### Return type

[**models::ApplicationDeploymentRestriction**](ApplicationDeploymentRestriction.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_application_deployment_restriction

> delete_application_deployment_restriction(application_id, deployment_restriction_id)
Delete an application deployment restriction

Delete an application deployment restriction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |
**deployment_restriction_id** | **uuid::Uuid** | Deployment Restriction ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_application_deployment_restriction

> models::ApplicationDeploymentRestriction edit_application_deployment_restriction(application_id, deployment_restriction_id, application_deployment_restriction_request)
Edit an application deployment restriction

Edit an application deployment restriction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |
**deployment_restriction_id** | **uuid::Uuid** | Deployment Restriction ID | [required] |
**application_deployment_restriction_request** | Option<[**ApplicationDeploymentRestrictionRequest**](ApplicationDeploymentRestrictionRequest.md)> |  |  |

### Return type

[**models::ApplicationDeploymentRestriction**](ApplicationDeploymentRestriction.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_application_deployment_restrictions

> models::ApplicationDeploymentRestrictionResponseList get_application_deployment_restrictions(application_id)
Get application deployment restrictions

Get application deployment restrictions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **uuid::Uuid** | Application ID | [required] |

### Return type

[**models::ApplicationDeploymentRestrictionResponseList**](ApplicationDeploymentRestrictionResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

