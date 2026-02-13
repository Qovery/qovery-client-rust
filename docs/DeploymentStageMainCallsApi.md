# \DeploymentStageMainCallsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**attach_service_to_deployment_stage**](DeploymentStageMainCallsApi.md#attach_service_to_deployment_stage) | **PUT** /deploymentStage/{deploymentStageId}/service/{serviceId} | Attach service to deployment stage
[**create_environment_deployment_stage**](DeploymentStageMainCallsApi.md#create_environment_deployment_stage) | **POST** /environment/{environmentId}/deploymentStage | Create environment deployment stage
[**delete_deployment_stage**](DeploymentStageMainCallsApi.md#delete_deployment_stage) | **DELETE** /deploymentStage/{deploymentStageId} | Delete deployment stage
[**edit_deployment_stage**](DeploymentStageMainCallsApi.md#edit_deployment_stage) | **PUT** /deploymentStage/{deploymentStageId} | Edit deployment stage
[**get_deployment_stage**](DeploymentStageMainCallsApi.md#get_deployment_stage) | **GET** /deploymentStage/{deploymentStageId} | Get Deployment Stage
[**get_service_deployment_stage**](DeploymentStageMainCallsApi.md#get_service_deployment_stage) | **GET** /service/{serviceId}/deploymentStage | Get Service Deployment Stage
[**list_environment_deployment_stage**](DeploymentStageMainCallsApi.md#list_environment_deployment_stage) | **GET** /environment/{environmentId}/deploymentStage | List environment deployment stage
[**move_after_deployment_stage**](DeploymentStageMainCallsApi.md#move_after_deployment_stage) | **PUT** /deploymentStage/{deploymentStageId}/moveAfter/{stageId} | Move deployment stage after requested stage
[**move_before_deployment_stage**](DeploymentStageMainCallsApi.md#move_before_deployment_stage) | **PUT** /deploymentStage/{deploymentStageId}/moveBefore/{stageId} | Move deployment stage before requested stage



## attach_service_to_deployment_stage

> models::DeploymentStageResponseList attach_service_to_deployment_stage(deployment_stage_id, service_id, attach_service_to_deployment_stage_request)
Attach service to deployment stage

Moves the service to the specified deployment stage. To skip a service from environment-level deployments while keeping it in its current stage, set `is_skipped: true` in the request body. Moving the service to a different stage automatically un-skips it (sets `is_skipped: false`).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_stage_id** | **uuid::Uuid** | Deployment Stage ID | [required] |
**service_id** | **uuid::Uuid** | Service ID of an application/job/container/database | [required] |
**attach_service_to_deployment_stage_request** | Option<[**AttachServiceToDeploymentStageRequest**](AttachServiceToDeploymentStageRequest.md)> |  |  |

### Return type

[**models::DeploymentStageResponseList**](DeploymentStageResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_environment_deployment_stage

> models::DeploymentStageResponse create_environment_deployment_stage(environment_id, deployment_stage_request)
Create environment deployment stage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**deployment_stage_request** | Option<[**DeploymentStageRequest**](DeploymentStageRequest.md)> |  |  |

### Return type

[**models::DeploymentStageResponse**](DeploymentStageResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_deployment_stage

> delete_deployment_stage(deployment_stage_id)
Delete deployment stage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_stage_id** | **uuid::Uuid** | Deployment Stage ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_deployment_stage

> models::DeploymentStageResponse edit_deployment_stage(deployment_stage_id, deployment_stage_request)
Edit deployment stage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_stage_id** | **uuid::Uuid** | Deployment Stage ID | [required] |
**deployment_stage_request** | Option<[**DeploymentStageRequest**](DeploymentStageRequest.md)> |  |  |

### Return type

[**models::DeploymentStageResponse**](DeploymentStageResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deployment_stage

> models::DeploymentStageResponse get_deployment_stage(deployment_stage_id)
Get Deployment Stage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_stage_id** | **uuid::Uuid** | Deployment Stage ID | [required] |

### Return type

[**models::DeploymentStageResponse**](DeploymentStageResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_deployment_stage

> models::DeploymentStageResponse get_service_deployment_stage(service_id)
Get Service Deployment Stage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **uuid::Uuid** | Service ID of an application/job/container/database | [required] |

### Return type

[**models::DeploymentStageResponse**](DeploymentStageResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_environment_deployment_stage

> models::DeploymentStageResponseList list_environment_deployment_stage(environment_id)
List environment deployment stage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |

### Return type

[**models::DeploymentStageResponseList**](DeploymentStageResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## move_after_deployment_stage

> models::DeploymentStageResponseList move_after_deployment_stage(deployment_stage_id, stage_id)
Move deployment stage after requested stage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_stage_id** | **uuid::Uuid** | Deployment Stage ID | [required] |
**stage_id** | **uuid::Uuid** | Deployment Stage ID | [required] |

### Return type

[**models::DeploymentStageResponseList**](DeploymentStageResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## move_before_deployment_stage

> models::DeploymentStageResponseList move_before_deployment_stage(deployment_stage_id, stage_id)
Move deployment stage before requested stage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_stage_id** | **uuid::Uuid** | Deployment Stage ID | [required] |
**stage_id** | **uuid::Uuid** | Deployment Stage ID | [required] |

### Return type

[**models::DeploymentStageResponseList**](DeploymentStageResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

