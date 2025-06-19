# \EnvironmentActionsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_environment_deployment**](EnvironmentActionsApi.md#cancel_environment_deployment) | **POST** /environment/{environmentId}/cancelDeployment | Cancel environment deployment
[**clean_failed_jobs**](EnvironmentActionsApi.md#clean_failed_jobs) | **POST** /environment/{environmentId}/cleanFailedJobs | Clean failed jobs within an environment
[**clone_environment**](EnvironmentActionsApi.md#clone_environment) | **POST** /environment/{environmentId}/clone | Clone environment
[**delete_selected_services**](EnvironmentActionsApi.md#delete_selected_services) | **POST** /environment/{environmentId}/service/delete | Delete services
[**deploy_all_services**](EnvironmentActionsApi.md#deploy_all_services) | **POST** /environment/{environmentId}/service/deploy | Deploy services
[**deploy_environment**](EnvironmentActionsApi.md#deploy_environment) | **POST** /environment/{environmentId}/deploy | Deploy environment
[**reboot_services**](EnvironmentActionsApi.md#reboot_services) | **POST** /environment/{environmentId}/service/restart-service | Reboot services
[**redeploy_environment**](EnvironmentActionsApi.md#redeploy_environment) | **POST** /environment/{environmentId}/redeploy | Redeploy environment
[**stop_environment**](EnvironmentActionsApi.md#stop_environment) | **POST** /environment/{environmentId}/stop | Stop environment
[**stop_selected_services**](EnvironmentActionsApi.md#stop_selected_services) | **POST** /environment/{environmentId}/service/stop | Stop services
[**uninstall_environment**](EnvironmentActionsApi.md#uninstall_environment) | **POST** /environment/{environmentId}/uninstall | Uninstall environment
[**uninstall_selected_services**](EnvironmentActionsApi.md#uninstall_selected_services) | **POST** /environment/{environmentId}/service/uninstall | Uninstall services



## cancel_environment_deployment

> models::EnvironmentStatus cancel_environment_deployment(environment_id, cancel_environment_deployment_request)
Cancel environment deployment

Cancel the current deployment of your environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**cancel_environment_deployment_request** | Option<[**CancelEnvironmentDeploymentRequest**](CancelEnvironmentDeploymentRequest.md)> |  |  |

### Return type

[**models::EnvironmentStatus**](EnvironmentStatus.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clean_failed_jobs

> models::CleanFailedJobs200Response clean_failed_jobs(environment_id, clean_failed_jobs_request)
Clean failed jobs within an environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **String** |  | [required] |
**clean_failed_jobs_request** | Option<[**CleanFailedJobsRequest**](CleanFailedJobsRequest.md)> |  |  |

### Return type

[**models::CleanFailedJobs200Response**](cleanFailedJobs_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clone_environment

> models::Environment clone_environment(environment_id, clone_environment_request)
Clone environment

You must provide a name. This will create a new environment, with the same configuration, and same applications and databases. Database data is not cloned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**clone_environment_request** | Option<[**CloneEnvironmentRequest**](CloneEnvironmentRequest.md)> |  |  |

### Return type

[**models::Environment**](Environment.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_selected_services

> delete_selected_services(environment_id, environment_service_ids_all_request)
Delete services

Delete selected services

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**environment_service_ids_all_request** | Option<[**EnvironmentServiceIdsAllRequest**](EnvironmentServiceIdsAllRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deploy_all_services

> models::EnvironmentStatus deploy_all_services(environment_id, deploy_all_request)
Deploy services

Update and deploy the selected services

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**deploy_all_request** | Option<[**DeployAllRequest**](DeployAllRequest.md)> |  |  |

### Return type

[**models::EnvironmentStatus**](EnvironmentStatus.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deploy_environment

> models::Status deploy_environment(environment_id)
Deploy environment

This will deploy all the services of this environment to their latest version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |

### Return type

[**models::Status**](Status.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reboot_services

> models::Status reboot_services(environment_id, reboot_services_request)
Reboot services

Update and reboot the selected services

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**reboot_services_request** | Option<[**RebootServicesRequest**](RebootServicesRequest.md)> |  |  |

### Return type

[**models::Status**](Status.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## redeploy_environment

> models::EnvironmentStatus redeploy_environment(environment_id)
Redeploy environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |

### Return type

[**models::EnvironmentStatus**](EnvironmentStatus.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_environment

> models::EnvironmentStatus stop_environment(environment_id)
Stop environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |

### Return type

[**models::EnvironmentStatus**](EnvironmentStatus.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_selected_services

> stop_selected_services(environment_id, environment_service_ids_all_request)
Stop services

Stop selected services

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**environment_service_ids_all_request** | Option<[**EnvironmentServiceIdsAllRequest**](EnvironmentServiceIdsAllRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uninstall_environment

> serde_json::Value uninstall_environment(environment_id)
Uninstall environment

This will uninstall all the services of this environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uninstall_selected_services

> uninstall_selected_services(environment_id, environment_service_ids_all_request)
Uninstall services

uninstall selected services

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**environment_service_ids_all_request** | Option<[**EnvironmentServiceIdsAllRequest**](EnvironmentServiceIdsAllRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

