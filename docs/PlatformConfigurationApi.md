# \PlatformConfigurationApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_cluster_platform_binding**](PlatformConfigurationApi.md#get_cluster_platform_binding) | **GET** /organization/{organizationId}/cluster/{clusterId}/platformBinding | Get the cluster platform binding
[**list_platform_templates**](PlatformConfigurationApi.md#list_platform_templates) | **GET** /organization/{organizationId}/platformTemplate | List platform templates
[**resolve_platform_component_configuration**](PlatformConfigurationApi.md#resolve_platform_component_configuration) | **POST** /organization/{organizationId}/cluster/{clusterId}/platformBinding/component/{componentKey}/resolve | Resolve a platform component configuration
[**resolve_platform_template_component_configuration**](PlatformConfigurationApi.md#resolve_platform_template_component_configuration) | **POST** /organization/{organizationId}/platformTemplate/{templateKey}/{templateVersion}/component/{componentKey}/resolve | Resolve a platform component configuration before cluster creation
[**update_cluster_platform_binding**](PlatformConfigurationApi.md#update_cluster_platform_binding) | **PUT** /organization/{organizationId}/cluster/{clusterId}/platformBinding | Update the cluster platform binding



## get_cluster_platform_binding

> models::ClusterPlatformBindingResponse get_cluster_platform_binding(organization_id, cluster_id)
Get the cluster platform binding

Returns the platform template selected for the cluster, its layer resolution, and the currently stored component configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |

### Return type

[**models::ClusterPlatformBindingResponse**](ClusterPlatformBindingResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_platform_templates

> models::PlatformTemplateCatalogResponse list_platform_templates(organization_id, cluster_mode, cloud_provider)
List platform templates

Returns the published platform templates available to the organization. Each template contains its layers, components, and the configuration fields that the Console can render. When clusterMode and cloudProvider are supplied together, component field constraints are narrowed to the effective choices for that cluster context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_mode** | Option<[**PlatformClusterMode**](PlatformClusterMode.md)> | Cluster management mode. Must be supplied together with cloudProvider. |  |
**cloud_provider** | Option<[**PlatformCloudVendor**](PlatformCloudVendor.md)> | Cluster cloud provider. Must be supplied together with clusterMode. |  |

### Return type

[**models::PlatformTemplateCatalogResponse**](PlatformTemplateCatalogResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resolve_platform_component_configuration

> models::PlatformComponentConfigurationPreviewResponse resolve_platform_component_configuration(organization_id, cluster_id, component_key, platform_component_configuration_preview_request)
Resolve a platform component configuration

Resolves the fields and runtime requirements to display for a component using the cluster context and the values currently entered in the Console. This operation is read-only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |
**component_key** | **String** | Platform component key | [required] |
**platform_component_configuration_preview_request** | [**PlatformComponentConfigurationPreviewRequest**](PlatformComponentConfigurationPreviewRequest.md) |  | [required] |

### Return type

[**models::PlatformComponentConfigurationPreviewResponse**](PlatformComponentConfigurationPreviewResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resolve_platform_template_component_configuration

> models::PlatformComponentConfigurationResolutionResponse resolve_platform_template_component_configuration(organization_id, template_key, template_version, component_key, cluster_mode, cloud_provider, platform_component_configuration_preview_request)
Resolve a platform component configuration before cluster creation

Resolves the fields and runtime requirements to display for a component using an explicit cluster context and the values currently entered in the Console. This operation is read-only and does not require an existing cluster or platform binding.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**template_key** | **String** | Platform template key | [required] |
**template_version** | **String** | Platform template version | [required] |
**component_key** | **String** | Platform component key | [required] |
**cluster_mode** | [**PlatformClusterMode**](PlatformClusterMode.md) | Cluster management mode used to resolve component applicability | [required] |
**cloud_provider** | [**PlatformCloudVendor**](PlatformCloudVendor.md) | Cluster cloud provider used to resolve component applicability | [required] |
**platform_component_configuration_preview_request** | [**PlatformComponentConfigurationPreviewRequest**](PlatformComponentConfigurationPreviewRequest.md) |  | [required] |

### Return type

[**models::PlatformComponentConfigurationResolutionResponse**](PlatformComponentConfigurationResolutionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_cluster_platform_binding

> models::ClusterPlatformBindingResponse update_cluster_platform_binding(organization_id, cluster_id, cluster_platform_binding_request)
Update the cluster platform binding

Selects a platform template and stores layer selections, component profile values, and customer-provided runtime inputs for the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |
**cluster_platform_binding_request** | [**ClusterPlatformBindingRequest**](ClusterPlatformBindingRequest.md) |  | [required] |

### Return type

[**models::ClusterPlatformBindingResponse**](ClusterPlatformBindingResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

