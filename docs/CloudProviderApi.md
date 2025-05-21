# \CloudProviderApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_aws_features**](CloudProviderApi.md#list_aws_features) | **GET** /aws/clusterFeature | List AWS features available
[**list_aws_instance_type**](CloudProviderApi.md#list_aws_instance_type) | **GET** /aws/instanceType | List AWS available instance types
[**list_aws_managed_database_instance_type**](CloudProviderApi.md#list_aws_managed_database_instance_type) | **GET** /aws/managedDatabase/instanceType/{region}/{databaseType} | List AWS available managed database instance types
[**list_aws_managed_database_type**](CloudProviderApi.md#list_aws_managed_database_type) | **GET** /aws/managedDatabase/type | List AWS available managed database types
[**list_aws_regions**](CloudProviderApi.md#list_aws_regions) | **GET** /aws/region | List AWS regions
[**list_awseks_instance_type**](CloudProviderApi.md#list_awseks_instance_type) | **GET** /aws/eks/instanceType/{region} | List AWS EKS available instance types
[**list_azure_aks_instance_type**](CloudProviderApi.md#list_azure_aks_instance_type) | **GET** /azure/aks/instanceType/{region} | List Azure AKS available instance types
[**list_azure_features**](CloudProviderApi.md#list_azure_features) | **GET** /azure/clusterFeature | List Azure features available
[**list_azure_regions**](CloudProviderApi.md#list_azure_regions) | **GET** /azure/region | List Azure regions
[**list_cloud_provider**](CloudProviderApi.md#list_cloud_provider) | **GET** /cloudProvider | List Cloud providers available
[**list_gcp_features**](CloudProviderApi.md#list_gcp_features) | **GET** /gcp/clusterFeature | List GCP features available
[**list_gcp_gke_instance_type**](CloudProviderApi.md#list_gcp_gke_instance_type) | **GET** /gcp/instanceType/{region} | List GCP GKE available instance types
[**list_gcp_regions**](CloudProviderApi.md#list_gcp_regions) | **GET** /gcp/region | List GCP regions
[**list_scaleway_features**](CloudProviderApi.md#list_scaleway_features) | **GET** /scaleway/clusterFeature | List Scaleway features available
[**list_scaleway_instance_type**](CloudProviderApi.md#list_scaleway_instance_type) | **GET** /scaleway/instanceType | List Scaleway available instance types
[**list_scaleway_kapsule_instance_type**](CloudProviderApi.md#list_scaleway_kapsule_instance_type) | **GET** /scaleway/instanceType/{zone} | List Scaleway Kapsule available instance types
[**list_scaleway_regions**](CloudProviderApi.md#list_scaleway_regions) | **GET** /scaleway/region | List Scaleway regions
[**list_scw_managed_database_instance_type**](CloudProviderApi.md#list_scw_managed_database_instance_type) | **GET** /scaleway/managedDatabase/instanceType/{zone}/{databaseType} | List Scaleway available managed database instance types
[**list_scw_managed_database_type**](CloudProviderApi.md#list_scw_managed_database_type) | **GET** /scaleway/managedDatabase/type | List Scaleway available managed database types



## list_aws_features

> models::ClusterFeatureResponseList list_aws_features()
List AWS features available

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ClusterFeatureResponseList**](ClusterFeatureResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_aws_instance_type

> models::ClusterInstanceTypeResponseList list_aws_instance_type()
List AWS available instance types

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ClusterInstanceTypeResponseList**](ClusterInstanceTypeResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_aws_managed_database_instance_type

> models::ManagedDatabaseInstanceTypeResponseList list_aws_managed_database_instance_type(region, database_type)
List AWS available managed database instance types

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | region name | [required] |
**database_type** | **String** | Database type | [required] |

### Return type

[**models::ManagedDatabaseInstanceTypeResponseList**](ManagedDatabaseInstanceTypeResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_aws_managed_database_type

> models::ManagedDatabaseTypeResponseList list_aws_managed_database_type()
List AWS available managed database types

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ManagedDatabaseTypeResponseList**](ManagedDatabaseTypeResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_aws_regions

> models::ClusterRegionResponseList list_aws_regions()
List AWS regions

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ClusterRegionResponseList**](ClusterRegionResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_awseks_instance_type

> models::ClusterInstanceTypeResponseList list_awseks_instance_type(region, only_meets_resource_reqs, with_gpu)
List AWS EKS available instance types

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | region name | [required] |
**only_meets_resource_reqs** | Option<**bool**> |  |  |
**with_gpu** | Option<**bool**> |  |  |

### Return type

[**models::ClusterInstanceTypeResponseList**](ClusterInstanceTypeResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_azure_aks_instance_type

> models::ClusterInstanceTypeResponseList list_azure_aks_instance_type(region, only_meets_resource_reqs, with_gpu)
List Azure AKS available instance types

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | region name | [required] |
**only_meets_resource_reqs** | Option<**bool**> |  |  |
**with_gpu** | Option<**bool**> |  |  |

### Return type

[**models::ClusterInstanceTypeResponseList**](ClusterInstanceTypeResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_azure_features

> models::ClusterFeatureResponseList list_azure_features()
List Azure features available

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ClusterFeatureResponseList**](ClusterFeatureResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_azure_regions

> models::ClusterRegionResponseList list_azure_regions()
List Azure regions

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ClusterRegionResponseList**](ClusterRegionResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cloud_provider

> models::CloudProviderResponseList list_cloud_provider()
List Cloud providers available

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CloudProviderResponseList**](CloudProviderResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gcp_features

> models::ClusterFeatureResponseList list_gcp_features()
List GCP features available

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ClusterFeatureResponseList**](ClusterFeatureResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gcp_gke_instance_type

> models::ClusterInstanceTypeResponseList list_gcp_gke_instance_type(region)
List GCP GKE available instance types

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | region name | [required] |

### Return type

[**models::ClusterInstanceTypeResponseList**](ClusterInstanceTypeResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gcp_regions

> models::ClusterRegionResponseList list_gcp_regions()
List GCP regions

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ClusterRegionResponseList**](ClusterRegionResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_scaleway_features

> models::ClusterFeatureResponseList list_scaleway_features()
List Scaleway features available

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ClusterFeatureResponseList**](ClusterFeatureResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_scaleway_instance_type

> models::ClusterInstanceTypeResponseList list_scaleway_instance_type()
List Scaleway available instance types

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ClusterInstanceTypeResponseList**](ClusterInstanceTypeResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_scaleway_kapsule_instance_type

> models::ClusterInstanceTypeResponseList list_scaleway_kapsule_instance_type(zone)
List Scaleway Kapsule available instance types

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | zone name | [required] |

### Return type

[**models::ClusterInstanceTypeResponseList**](ClusterInstanceTypeResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_scaleway_regions

> models::ClusterRegionResponseList list_scaleway_regions()
List Scaleway regions

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ClusterRegionResponseList**](ClusterRegionResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_scw_managed_database_instance_type

> models::ManagedDatabaseInstanceTypeResponseList list_scw_managed_database_instance_type(database_type)
List Scaleway available managed database instance types

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_type** | **String** | Database type | [required] |

### Return type

[**models::ManagedDatabaseInstanceTypeResponseList**](ManagedDatabaseInstanceTypeResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_scw_managed_database_type

> models::ManagedDatabaseTypeResponseList list_scw_managed_database_type()
List Scaleway available managed database types

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ManagedDatabaseTypeResponseList**](ManagedDatabaseTypeResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

