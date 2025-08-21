# \ClustersApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_cluster**](ClustersApi.md#create_cluster) | **POST** /organization/{organizationId}/cluster | Create a cluster
[**delete_cluster**](ClustersApi.md#delete_cluster) | **DELETE** /organization/{organizationId}/cluster/{clusterId} | Delete a cluster
[**deploy_cluster**](ClustersApi.md#deploy_cluster) | **POST** /organization/{organizationId}/cluster/{clusterId}/deploy | Deploy a cluster
[**edit_cluster**](ClustersApi.md#edit_cluster) | **PUT** /organization/{organizationId}/cluster/{clusterId} | Edit a cluster
[**edit_cluster_advanced_settings**](ClustersApi.md#edit_cluster_advanced_settings) | **PUT** /organization/{organizationId}/cluster/{clusterId}/advancedSettings | Edit advanced settings
[**edit_cluster_kubeconfig**](ClustersApi.md#edit_cluster_kubeconfig) | **PUT** /organization/{organizationId}/cluster/{clusterId}/kubeconfig | Edit cluster kubeconfig
[**edit_routing_table**](ClustersApi.md#edit_routing_table) | **PUT** /organization/{organizationId}/cluster/{clusterId}/routingTable | Edit routing table
[**get_cluster_advanced_settings**](ClustersApi.md#get_cluster_advanced_settings) | **GET** /organization/{organizationId}/cluster/{clusterId}/advancedSettings | Get advanced settings
[**get_cluster_kubeconfig**](ClustersApi.md#get_cluster_kubeconfig) | **GET** /organization/{organizationId}/cluster/{clusterId}/kubeconfig | Get cluster kubeconfig
[**get_cluster_kubernetes_events**](ClustersApi.md#get_cluster_kubernetes_events) | **GET** /cluster/{clusterId}/events | List Cluster Kubernetes Events
[**get_cluster_metrics**](ClustersApi.md#get_cluster_metrics) | **GET** /cluster/{clusterId}/metrics | Fetch cluster metrics
[**get_cluster_readiness_status**](ClustersApi.md#get_cluster_readiness_status) | **GET** /organization/{organizationId}/cluster/{clusterId}/isReady | Know if a cluster is ready to be deployed or not
[**get_cluster_status**](ClustersApi.md#get_cluster_status) | **GET** /organization/{organizationId}/cluster/{clusterId}/status | Get cluster status
[**get_default_cluster_advanced_settings**](ClustersApi.md#get_default_cluster_advanced_settings) | **GET** /defaultClusterAdvancedSettings | List default cluster advanced settings
[**get_installation_helm_values**](ClustersApi.md#get_installation_helm_values) | **GET** /organization/{organizationId}/cluster/{clusterId}/installationHelmValues | Get cluster helm values for self managed installation
[**get_organization_cloud_provider_info**](ClustersApi.md#get_organization_cloud_provider_info) | **GET** /organization/{organizationId}/cluster/{clusterId}/cloudProviderInfo | Get cluster cloud provider info and credentials
[**get_organization_cluster_status**](ClustersApi.md#get_organization_cluster_status) | **GET** /organization/{organizationId}/cluster/status | List all clusters statuses
[**get_routing_table**](ClustersApi.md#get_routing_table) | **GET** /organization/{organizationId}/cluster/{clusterId}/routingTable | Get routing table
[**list_cluster_logs**](ClustersApi.md#list_cluster_logs) | **GET** /organization/{organizationId}/cluster/{clusterId}/logs | List Cluster Logs
[**list_organization_cluster**](ClustersApi.md#list_organization_cluster) | **GET** /organization/{organizationId}/cluster | List organization clusters
[**lock_cluster**](ClustersApi.md#lock_cluster) | **POST** /cluster/{clusterId}/lock | Lock Cluster
[**specify_cluster_cloud_provider_info**](ClustersApi.md#specify_cluster_cloud_provider_info) | **POST** /organization/{organizationId}/cluster/{clusterId}/cloudProviderInfo | Specify cluster cloud provider info and credentials
[**stop_cluster**](ClustersApi.md#stop_cluster) | **POST** /organization/{organizationId}/cluster/{clusterId}/stop | Stop cluster
[**unlock_cluster**](ClustersApi.md#unlock_cluster) | **DELETE** /cluster/{clusterId}/lock | Unlock Cluster
[**update_karpenter_private_fargate_subnet_ids**](ClustersApi.md#update_karpenter_private_fargate_subnet_ids) | **PUT** /organization/{organizationId}/cluster/{clusterId}/karpenterPrivateSubnetIds | Update karpenter private fargate subnet ids
[**upgrade_cluster**](ClustersApi.md#upgrade_cluster) | **POST** /cluster/{clusterId}/upgrade | Upgrade a cluster



## create_cluster

> models::Cluster create_cluster(organization_id, cluster_request)
Create a cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_request** | Option<[**ClusterRequest**](ClusterRequest.md)> |  |  |

### Return type

[**models::Cluster**](Cluster.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_cluster

> delete_cluster(organization_id, cluster_id, delete_mode)
Delete a cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |
**delete_mode** | Option<[**ClusterDeleteMode**](.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deploy_cluster

> models::ClusterStatus deploy_cluster(organization_id, cluster_id, dry_run)
Deploy a cluster

allows to deploy a cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |
**dry_run** | Option<**bool**> | default: false. Decide if the deployment of the cluster should be done in dry_run mode. Avoiding any changes |  |

### Return type

[**models::ClusterStatus**](ClusterStatus.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_cluster

> models::Cluster edit_cluster(organization_id, cluster_id, cluster_request)
Edit a cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |
**cluster_request** | Option<[**ClusterRequest**](ClusterRequest.md)> |  |  |

### Return type

[**models::Cluster**](Cluster.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_cluster_advanced_settings

> models::ClusterAdvancedSettings edit_cluster_advanced_settings(organization_id, cluster_id, cluster_advanced_settings)
Edit advanced settings

Edit advanced settings by returning table of advanced settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |
**cluster_advanced_settings** | Option<[**ClusterAdvancedSettings**](ClusterAdvancedSettings.md)> |  |  |

### Return type

[**models::ClusterAdvancedSettings**](ClusterAdvancedSettings.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_cluster_kubeconfig

> edit_cluster_kubeconfig(organization_id, cluster_id, body)
Edit cluster kubeconfig

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |
**body** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/x-yaml
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_routing_table

> models::ClusterRoutingTable edit_routing_table(organization_id, cluster_id, cluster_routing_table_request)
Edit routing table

Edit routing table by returning updated table.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |
**cluster_routing_table_request** | Option<[**ClusterRoutingTableRequest**](ClusterRoutingTableRequest.md)> |  |  |

### Return type

[**models::ClusterRoutingTable**](ClusterRoutingTable.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_advanced_settings

> models::ClusterAdvancedSettings get_cluster_advanced_settings(organization_id, cluster_id)
Get advanced settings

Get the list and values of the advanced settings of the cluster. Default values for each setting are available in [our documentation](https://hub.qovery.com/docs/using-qovery/configuration/cluster-advanced-settings/) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |

### Return type

[**models::ClusterAdvancedSettings**](ClusterAdvancedSettings.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_kubeconfig

> String get_cluster_kubeconfig(organization_id, cluster_id, with_token_from_cli)
Get cluster kubeconfig

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |
**with_token_from_cli** | Option<**bool**> | If true, the user auth part will have an exec command with qovery cli |  |

### Return type

**String**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_kubernetes_events

> models::GetClusterKubernetesEvents200Response get_cluster_kubernetes_events(cluster_id, from_date_time, to_date_time, node_name, pod_name, reporting_component)
List Cluster Kubernetes Events

List Cluster Kubernetes Events

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |
**from_date_time** | **String** | The start date time to fetch events from, following ISO-8601 format.   The `+` character must be escaped (`%2B`)  | [required] |
**to_date_time** | **String** | The end date time to fetch events from, following ISO-8601 format.   The `+` character must be escaped (`%2B`)  | [required] |
**node_name** | Option<**String**> | The name of the node to fetch event from |  |
**pod_name** | Option<**String**> | The name of the pod to fetch event from |  |
**reporting_component** | Option<**String**> | The name of the reporting component used to filter events. |  |

### Return type

[**models::GetClusterKubernetesEvents200Response**](get_cluster_kubernetes_events_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_metrics

> models::ClusterMetricsResponse get_cluster_metrics(cluster_id, endpoint, query, start, end, step, time, timeout, dedup, partial_response, max_source_resolution)
Fetch cluster metrics

Fetch cluster metrics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |
**endpoint** | **String** |  | [required] |
**query** | **String** |  | [required] |
**start** | Option<**String**> |  |  |
**end** | Option<**String**> |  |  |
**step** | Option<**String**> |  |  |
**time** | Option<**String**> |  |  |
**timeout** | Option<**String**> |  |  |
**dedup** | Option<**String**> |  |  |
**partial_response** | Option<**String**> |  |  |
**max_source_resolution** | Option<**String**> |  |  |

### Return type

[**models::ClusterMetricsResponse**](ClusterMetricsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_readiness_status

> models::ClusterReadinessStatus get_cluster_readiness_status(organization_id, cluster_id)
Know if a cluster is ready to be deployed or not

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |

### Return type

[**models::ClusterReadinessStatus**](ClusterReadinessStatus.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_status

> models::ClusterStatus get_cluster_status(organization_id, cluster_id)
Get cluster status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |

### Return type

[**models::ClusterStatus**](ClusterStatus.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_cluster_advanced_settings

> models::ClusterAdvancedSettings get_default_cluster_advanced_settings()
List default cluster advanced settings

Default values for each setting are available in [our documentation](https://hub.qovery.com/docs/using-qovery/configuration/cluster-advanced-settings/)

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ClusterAdvancedSettings**](ClusterAdvancedSettings.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_installation_helm_values

> String get_installation_helm_values(organization_id, cluster_id)
Get cluster helm values for self managed installation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |

### Return type

**String**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_cloud_provider_info

> models::ClusterCloudProviderInfo get_organization_cloud_provider_info(organization_id, cluster_id)
Get cluster cloud provider info and credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |

### Return type

[**models::ClusterCloudProviderInfo**](ClusterCloudProviderInfo.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_cluster_status

> models::ClusterStatusResponseList get_organization_cluster_status(organization_id)
List all clusters statuses

Returns a list of clusters with only their id and status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::ClusterStatusResponseList**](ClusterStatusResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_table

> models::ClusterRoutingTable get_routing_table(organization_id, cluster_id)
Get routing table

Retrieve network routing table where each line corresponds to a route between a destination and a target.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |

### Return type

[**models::ClusterRoutingTable**](ClusterRoutingTable.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cluster_logs

> models::ClusterLogsResponseList list_cluster_logs(organization_id, cluster_id)
List Cluster Logs

List Cluster Logs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |

### Return type

[**models::ClusterLogsResponseList**](ClusterLogsResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_cluster

> models::ClusterResponseList list_organization_cluster(organization_id)
List organization clusters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::ClusterResponseList**](ClusterResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lock_cluster

> models::ClusterLock lock_cluster(cluster_id, cluster_lock_request)
Lock Cluster

Lock a Cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **String** |  | [required] |
**cluster_lock_request** | Option<[**ClusterLockRequest**](ClusterLockRequest.md)> |  |  |

### Return type

[**models::ClusterLock**](ClusterLock.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## specify_cluster_cloud_provider_info

> models::ClusterCloudProviderInfo specify_cluster_cloud_provider_info(organization_id, cluster_id, cluster_cloud_provider_info_request)
Specify cluster cloud provider info and credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |
**cluster_cloud_provider_info_request** | Option<[**ClusterCloudProviderInfoRequest**](ClusterCloudProviderInfoRequest.md)> |  |  |

### Return type

[**models::ClusterCloudProviderInfo**](ClusterCloudProviderInfo.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_cluster

> models::ClusterStatus stop_cluster(organization_id, cluster_id)
Stop cluster

Cluster stop has been requester.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |

### Return type

[**models::ClusterStatus**](ClusterStatus.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlock_cluster

> unlock_cluster(cluster_id)
Unlock Cluster

Unlock a cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_karpenter_private_fargate_subnet_ids

> update_karpenter_private_fargate_subnet_ids(organization_id, cluster_id, cluster_karpenter_private_subnet_ids_put_request)
Update karpenter private fargate subnet ids

Update karpenter private fargate subnet ids

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |
**cluster_karpenter_private_subnet_ids_put_request** | Option<[**ClusterKarpenterPrivateSubnetIdsPutRequest**](ClusterKarpenterPrivateSubnetIdsPutRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upgrade_cluster

> models::ClusterStatus upgrade_cluster(cluster_id)
Upgrade a cluster

allows to upgrade a cluster to next available kubernetes version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |

### Return type

[**models::ClusterStatus**](ClusterStatus.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

