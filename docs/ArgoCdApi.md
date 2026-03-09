# \ArgoCdApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_argo_cd_connection**](ArgoCdApi.md#check_argo_cd_connection) | **POST** /cluster/{clusterId}/argoCdConfig/check | Check ArgoCD connection
[**delete_argo_cd_credentials**](ArgoCdApi.md#delete_argo_cd_credentials) | **DELETE** /cluster/{clusterId}/argoCdConfig | Delete ArgoCD credentials for a cluster
[**get_argo_cd_credentials**](ArgoCdApi.md#get_argo_cd_credentials) | **GET** /cluster/{clusterId}/argoCdConfig | Get ArgoCD credentials for a cluster
[**save_argo_cd_credentials**](ArgoCdApi.md#save_argo_cd_credentials) | **POST** /cluster/{clusterId}/argoCdConfig | Save ArgoCD credentials for a cluster



## check_argo_cd_connection

> models::ArgoCdConnectionCheckResponse check_argo_cd_connection(cluster_id, argo_cd_credentials_request)
Check ArgoCD connection

Test an ArgoCD URL and token before saving. The cluster agent attempts to connect to ArgoCD and returns the connection result. Always returns HTTP 200 — check the `status` field for the connection outcome. Requires ADMIN role. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |
**argo_cd_credentials_request** | [**ArgoCdCredentialsRequest**](ArgoCdCredentialsRequest.md) |  | [required] |

### Return type

[**models::ArgoCdConnectionCheckResponse**](ArgoCdConnectionCheckResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_argo_cd_credentials

> delete_argo_cd_credentials(cluster_id)
Delete ArgoCD credentials for a cluster

Remove the stored ArgoCD configuration for a cluster. Requires ADMIN role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_argo_cd_credentials

> models::ArgoCdCredentialsResponse get_argo_cd_credentials(cluster_id)
Get ArgoCD credentials for a cluster

Retrieve the stored ArgoCD configuration for a cluster. The token is always returned as REDACTED. Requires VIEWER role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |

### Return type

[**models::ArgoCdCredentialsResponse**](ArgoCdCredentialsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_argo_cd_credentials

> models::ArgoCdCredentialsResponse save_argo_cd_credentials(cluster_id, argo_cd_credentials_request)
Save ArgoCD credentials for a cluster

Save or update the ArgoCD URL and authentication token for a cluster. Requires ADMIN role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |
**argo_cd_credentials_request** | [**ArgoCdCredentialsRequest**](ArgoCdCredentialsRequest.md) |  | [required] |

### Return type

[**models::ArgoCdCredentialsResponse**](ArgoCdCredentialsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

