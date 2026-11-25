# \ServiceMainCallsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_service_deployment_restriction**](ServiceMainCallsApi.md#create_service_deployment_restriction) | **POST** /service/{serviceId}/deploymentRestriction | Create a service deployment restriction
[**delete_service_deployment_restriction**](ServiceMainCallsApi.md#delete_service_deployment_restriction) | **DELETE** /service/{serviceId}/deploymentRestriction/{deploymentRestrictionId} | Delete a service deployment restriction
[**edit_service_deployment_restriction**](ServiceMainCallsApi.md#edit_service_deployment_restriction) | **PUT** /service/{serviceId}/deploymentRestriction/{deploymentRestrictionId} | Edit a service deployment restriction
[**get_service_git_webhook_status**](ServiceMainCallsApi.md#get_service_git_webhook_status) | **GET** /service/{serviceId}/gitWebhookStatus | Get git webhook status for a service
[**list_service_deployment_restrictions**](ServiceMainCallsApi.md#list_service_deployment_restrictions) | **GET** /service/{serviceId}/deploymentRestriction | List service deployment restrictions
[**sync_service_git_webhook**](ServiceMainCallsApi.md#sync_service_git_webhook) | **POST** /service/{serviceId}/gitWebhook/sync | Synchronize git webhook for a service



## create_service_deployment_restriction

> models::ServiceDeploymentRestrictionResponse create_service_deployment_restriction(service_id, service_deployment_restriction_request)
Create a service deployment restriction

Creates a deployment restriction on a service. Works for applications, jobs, Helm, Terraform and containers. Restrictions filter which pushes to the source branch deploy the service, by changed file path.  Returns 400 when the service cannot have restrictions: a job or container with an image source, a Helm or Terraform service not using a Git repository, or a kind without restrictions: a database or an agentic workflow. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **uuid::Uuid** | Service ID of an application/job/container/database | [required] |
**service_deployment_restriction_request** | Option<[**ServiceDeploymentRestrictionRequest**](ServiceDeploymentRestrictionRequest.md)> |  |  |

### Return type

[**models::ServiceDeploymentRestrictionResponse**](ServiceDeploymentRestrictionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_service_deployment_restriction

> delete_service_deployment_restriction(service_id, deployment_restriction_id)
Delete a service deployment restriction

Deletes a deployment restriction of a service. Works for applications, jobs, Helm, Terraform and containers.  Works on any container, whatever its source: a container's restrictions survive a switch to an image source, without effect until it switches back to a Git repository source.  Returns 400 when the service is a kind that cannot have deployment restrictions: a database or an agentic workflow. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **uuid::Uuid** | Service ID of an application/job/container/database | [required] |
**deployment_restriction_id** | **uuid::Uuid** | Deployment Restriction ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_service_deployment_restriction

> models::ServiceDeploymentRestrictionResponse edit_service_deployment_restriction(service_id, deployment_restriction_id, service_deployment_restriction_request)
Edit a service deployment restriction

Edits a deployment restriction of a service. Works for applications, jobs, Helm, Terraform and containers.  Returns 400 when the service is a container with an image source, or a kind that cannot have deployment restrictions: a database or an agentic workflow. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **uuid::Uuid** | Service ID of an application/job/container/database | [required] |
**deployment_restriction_id** | **uuid::Uuid** | Deployment Restriction ID | [required] |
**service_deployment_restriction_request** | Option<[**ServiceDeploymentRestrictionRequest**](ServiceDeploymentRestrictionRequest.md)> |  |  |

### Return type

[**models::ServiceDeploymentRestrictionResponse**](ServiceDeploymentRestrictionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_git_webhook_status

> models::GitWebhookStatusResponse get_service_git_webhook_status(service_id)
Get git webhook status for a service

Returns the webhook status for a git-based service. Checks if the Qovery webhook is correctly configured on the git provider (GitHub, GitLab, or Bitbucket).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **uuid::Uuid** | Service ID of an application/job/container/database | [required] |

### Return type

[**models::GitWebhookStatusResponse**](GitWebhookStatusResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_service_deployment_restrictions

> models::ServiceDeploymentRestrictionResponseList list_service_deployment_restrictions(service_id)
List service deployment restrictions

Lists the deployment restrictions of a service. Works for applications, jobs, Helm, Terraform and containers. Restrictions filter which pushes to the source branch deploy the service, by changed file path.  Works on any container, whatever its source: a container's restrictions survive a switch to an image source, without effect until it switches back to a Git repository source.  Returns 400 when the service is a kind that cannot have deployment restrictions: a database or an agentic workflow. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **uuid::Uuid** | Service ID of an application/job/container/database | [required] |

### Return type

[**models::ServiceDeploymentRestrictionResponseList**](ServiceDeploymentRestrictionResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_service_git_webhook

> models::GitWebhookStatusResponse sync_service_git_webhook(service_id)
Synchronize git webhook for a service

Synchronizes the Qovery webhook configuration for a git-based service. - If no webhook exists, creates one with the correct configuration - If webhook exists but is misconfigured, updates it with the correct events - If webhook is already correctly configured, returns current status Works with GitHub, GitLab, and Bitbucket repositories. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **uuid::Uuid** | Service ID of an application/job/container/database | [required] |

### Return type

[**models::GitWebhookStatusResponse**](GitWebhookStatusResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

