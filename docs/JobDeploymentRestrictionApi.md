# \JobDeploymentRestrictionApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_job_deployment_restriction**](JobDeploymentRestrictionApi.md#create_job_deployment_restriction) | **POST** /job/{jobId}/deploymentRestriction | Create a job deployment restriction
[**delete_job_deployment_restriction**](JobDeploymentRestrictionApi.md#delete_job_deployment_restriction) | **DELETE** /job/{jobId}/deploymentRestriction/{deploymentRestrictionId} | Delete a job deployment restriction
[**edit_job_deployment_restriction**](JobDeploymentRestrictionApi.md#edit_job_deployment_restriction) | **PUT** /job/{jobId}/deploymentRestriction/{deploymentRestrictionId} | Edit a job deployment restriction
[**get_job_deployment_restrictions**](JobDeploymentRestrictionApi.md#get_job_deployment_restrictions) | **GET** /job/{jobId}/deploymentRestriction | Get job deployment restrictions



## create_job_deployment_restriction

> models::JobDeploymentRestrictionResponse create_job_deployment_restriction(job_id, job_deployment_restriction_request)
Create a job deployment restriction

Create a job deployment restriction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |
**job_deployment_restriction_request** | Option<[**JobDeploymentRestrictionRequest**](JobDeploymentRestrictionRequest.md)> |  |  |

### Return type

[**models::JobDeploymentRestrictionResponse**](JobDeploymentRestrictionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_job_deployment_restriction

> delete_job_deployment_restriction(job_id, deployment_restriction_id)
Delete a job deployment restriction

Delete a job deployment restriction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |
**deployment_restriction_id** | **uuid::Uuid** | Deployment Restriction ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_job_deployment_restriction

> models::JobDeploymentRestrictionResponse edit_job_deployment_restriction(job_id, deployment_restriction_id, job_deployment_restriction_request)
Edit a job deployment restriction

Edit a job deployment restriction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |
**deployment_restriction_id** | **uuid::Uuid** | Deployment Restriction ID | [required] |
**job_deployment_restriction_request** | Option<[**JobDeploymentRestrictionRequest**](JobDeploymentRestrictionRequest.md)> |  |  |

### Return type

[**models::JobDeploymentRestrictionResponse**](JobDeploymentRestrictionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_deployment_restrictions

> models::JobDeploymentRestrictionResponseList get_job_deployment_restrictions(job_id)
Get job deployment restrictions

Get job deployment restrictions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |

### Return type

[**models::JobDeploymentRestrictionResponseList**](JobDeploymentRestrictionResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

