# \ArgoCdApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_argo_cd_connection**](ArgoCdApi.md#check_argo_cd_connection) | **POST** /cluster/{clusterId}/argoCdConfig/check | Check ArgoCD connection
[**delete_argo_cd_credentials**](ArgoCdApi.md#delete_argo_cd_credentials) | **DELETE** /cluster/{clusterId}/argoCdConfig | Delete ArgoCD credentials for a cluster
[**delete_argo_cd_destination_cluster_mapping**](ArgoCdApi.md#delete_argo_cd_destination_cluster_mapping) | **DELETE** /organization/{organizationId}/argoCdDestinationClusterMapping | Delete an ArgoCD destination cluster mapping
[**get_argo_cd_app**](ArgoCdApi.md#get_argo_cd_app) | **GET** /argocdApp/{argocdAppId} | Get ArgoCD app by ID
[**get_argo_cd_app_manifest**](ArgoCdApi.md#get_argo_cd_app_manifest) | **GET** /argocdApp/{argocdAppId}/manifest | Get ArgoCD app manifest enrichment
[**get_argo_cd_associated_services**](ArgoCdApi.md#get_argo_cd_associated_services) | **GET** /cluster/{clusterId}/argocdApps/associatedServices | Get ArgoCD associated services for a cluster
[**get_argo_cd_credentials**](ArgoCdApi.md#get_argo_cd_credentials) | **GET** /cluster/{clusterId}/argoCdConfig | Get ArgoCD credentials for a cluster
[**list_argo_cd_destination_cluster_mappings**](ArgoCdApi.md#list_argo_cd_destination_cluster_mappings) | **GET** /organization/{organizationId}/argoCdDestinationClusterMapping | List ArgoCD instance mappings for an organization
[**save_argo_cd_credentials**](ArgoCdApi.md#save_argo_cd_credentials) | **POST** /cluster/{clusterId}/argoCdConfig | Save ArgoCD credentials for a cluster
[**save_argo_cd_destination_cluster_mapping**](ArgoCdApi.md#save_argo_cd_destination_cluster_mapping) | **POST** /organization/{organizationId}/argoCdDestinationClusterMapping | Save an ArgoCD destination cluster mapping



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


## delete_argo_cd_destination_cluster_mapping

> delete_argo_cd_destination_cluster_mapping(organization_id, agent_cluster_id, argocd_cluster_url)
Delete an ArgoCD destination cluster mapping

Remove the mapping between an ArgoCD destination cluster URL and a Qovery cluster. Requires ADMIN role on the agent cluster. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**agent_cluster_id** | **uuid::Uuid** | ID of the Qovery cluster where the ArgoCD instance is running | [required] |
**argocd_cluster_url** | **String** | ArgoCD destination cluster URL as reported by ArgoCD | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_argo_cd_app

> models::ArgocdAppResponse get_argo_cd_app(argocd_app_id)
Get ArgoCD app by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**argocd_app_id** | **uuid::Uuid** | ArgoCD App ID | [required] |

### Return type

[**models::ArgocdAppResponse**](ArgocdAppResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_argo_cd_app_manifest

> models::ArgocdAppManifestResponse get_argo_cd_app_manifest(argocd_app_id)
Get ArgoCD app manifest enrichment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**argocd_app_id** | **uuid::Uuid** | ArgoCD App ID | [required] |

### Return type

[**models::ArgocdAppManifestResponse**](ArgocdAppManifestResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_argo_cd_associated_services

> models::ArgocdAssociatedServicesResponseList get_argo_cd_associated_services(cluster_id)
Get ArgoCD associated services for a cluster

List the ArgoCD discovered apps for a cluster, mapped to their project, environment, and service context. Requires VIEWER role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |

### Return type

[**models::ArgocdAssociatedServicesResponseList**](ArgocdAssociatedServicesResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

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


## list_argo_cd_destination_cluster_mappings

> models::ArgoCdInstanceMappingResponseList list_argo_cd_destination_cluster_mappings(organization_id)
List ArgoCD instance mappings for an organization

Returns one entry per ArgoCD agent cluster that has credentials configured. Each entry lists linked clusters and unlinked clusters. Requires VIEWER role. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::ArgoCdInstanceMappingResponseList**](ArgoCdInstanceMappingResponseList.md)

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


## save_argo_cd_destination_cluster_mapping

> models::ArgoCdDestinationClusterMappingResponse save_argo_cd_destination_cluster_mapping(organization_id, argo_cd_destination_cluster_mapping_request)
Save an ArgoCD destination cluster mapping

Map an ArgoCD destination cluster URL to a Qovery cluster for a given agent cluster. If a mapping for the same (agentClusterId, argocdClusterUrl) already exists, it is updated. Requires ADMIN role on the agent cluster. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**argo_cd_destination_cluster_mapping_request** | [**ArgoCdDestinationClusterMappingRequest**](ArgoCdDestinationClusterMappingRequest.md) |  | [required] |

### Return type

[**models::ArgoCdDestinationClusterMappingResponse**](ArgoCdDestinationClusterMappingResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

