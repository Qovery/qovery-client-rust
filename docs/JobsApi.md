# \JobsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auto_deploy_job_environments**](JobsApi.md#auto_deploy_job_environments) | **POST** /organization/{organizationId}/job/deploy | Auto deploy jobs
[**clone_job**](JobsApi.md#clone_job) | **POST** /job/{jobId}/clone | Clone job
[**create_job**](JobsApi.md#create_job) | **POST** /environment/{environmentId}/job | Create a job
[**get_default_job_advanced_settings**](JobsApi.md#get_default_job_advanced_settings) | **GET** /defaultJobAdvancedSettings | List default job advanced settings
[**get_environment_job_status**](JobsApi.md#get_environment_job_status) | **GET** /environment/{environmentId}/job/status | List all environment job statuses
[**list_jobs**](JobsApi.md#list_jobs) | **GET** /environment/{environmentId}/job | List jobs



## auto_deploy_job_environments

> models::Status auto_deploy_job_environments(organization_id, organization_job_auto_deploy_request)
Auto deploy jobs

Triggers a new job deploy in each environment matching the following conditions - environment should have the auto-deploy enabled - the job should have the same image name and a different tag 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**organization_job_auto_deploy_request** | Option<[**OrganizationJobAutoDeployRequest**](OrganizationJobAutoDeployRequest.md)> |  |  |

### Return type

[**models::Status**](Status.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clone_job

> models::JobResponse clone_job(job_id, clone_service_request)
Clone job

This will create a new job with the same configuration on the targeted environment Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Job ID | [required] |
**clone_service_request** | Option<[**CloneServiceRequest**](CloneServiceRequest.md)> |  |  |

### Return type

[**models::JobResponse**](JobResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_job

> models::JobResponse create_job(environment_id, job_request)
Create a job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**job_request** | Option<[**JobRequest**](JobRequest.md)> |  |  |

### Return type

[**models::JobResponse**](JobResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_job_advanced_settings

> models::JobAdvancedSettings get_default_job_advanced_settings()
List default job advanced settings

Default values for each setting is available in [our documentation](https://hub.qovery.com/docs/using-qovery/configuration/advanced-settings/)

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::JobAdvancedSettings**](JobAdvancedSettings.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_environment_job_status

> models::ReferenceObjectStatusResponseList get_environment_job_status(environment_id)
List all environment job statuses

Returns a list of jobs with only their id and status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |

### Return type

[**models::ReferenceObjectStatusResponseList**](ReferenceObjectStatusResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_jobs

> models::JobResponseList list_jobs(environment_id, to_update)
List jobs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**to_update** | Option<**bool**> | return (or not) results that must be updated |  |[default to false]

### Return type

[**models::JobResponseList**](JobResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

