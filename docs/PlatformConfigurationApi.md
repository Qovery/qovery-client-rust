# \PlatformConfigurationApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_cluster_platform_binding**](PlatformConfigurationApi.md#get_cluster_platform_binding) | **GET** /organization/{organizationId}/cluster/{clusterId}/platformBinding | Get the cluster platform binding
[**get_cluster_platform_configuration**](PlatformConfigurationApi.md#get_cluster_platform_configuration) | **GET** /v1/cluster/{clusterId}/platformConfiguration | Get the cluster platform configuration
[**list_platform_templates**](PlatformConfigurationApi.md#list_platform_templates) | **GET** /organization/{organizationId}/platformTemplate | List platform templates
[**resolve_cluster_platform_component_configuration**](PlatformConfigurationApi.md#resolve_cluster_platform_component_configuration) | **POST** /v1/cluster/{clusterId}/platformConfiguration/component/{componentKey}/resolve | Resolve a platform component configuration of the cluster
[**resolve_platform_component_configuration**](PlatformConfigurationApi.md#resolve_platform_component_configuration) | **POST** /organization/{organizationId}/cluster/{clusterId}/platformBinding/component/{componentKey}/resolve | Resolve a platform component configuration
[**resolve_platform_template_component_configuration**](PlatformConfigurationApi.md#resolve_platform_template_component_configuration) | **POST** /organization/{organizationId}/platformTemplate/{templateKey}/{templateVersion}/component/{componentKey}/resolve | Resolve a platform component configuration before cluster creation
[**update_cluster_platform_binding**](PlatformConfigurationApi.md#update_cluster_platform_binding) | **PUT** /organization/{organizationId}/cluster/{clusterId}/platformBinding | Update the cluster platform binding
[**update_cluster_platform_configuration**](PlatformConfigurationApi.md#update_cluster_platform_configuration) | **PUT** /v1/cluster/{clusterId}/platformConfiguration | Update the cluster platform configuration



## get_cluster_platform_binding

> models::ClusterPlatformBindingResponse get_cluster_platform_binding(organization_id, cluster_id)
Get the cluster platform binding

Returns the platform template selected for the cluster, its layer resolution, and the currently stored component configuration. Deprecated: use getClusterPlatformConfiguration instead.

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


## get_cluster_platform_configuration

> models::ClusterPlatformConfigurationResponse get_cluster_platform_configuration(cluster_id)
Get the cluster platform configuration

Returns the platform selection of the cluster (template release, layer selections and component configuration), its cluster inputs and the resolution of each layer. Sensitive managedConfig values are returned as `\"<redacted>\"`. Cluster inputs are identifiers, never secrets, and are returned as stored.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |

### Return type

[**models::ClusterPlatformConfigurationResponse**](ClusterPlatformConfigurationResponse.md)

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


## resolve_cluster_platform_component_configuration

> models::PlatformComponentConfigurationPreviewResponse resolve_cluster_platform_component_configuration(cluster_id, component_key, platform_component_configuration_preview_request)
Resolve a platform component configuration of the cluster

Resolves the fields and runtime requirements of a component from the cluster context, its stored platform configuration (the default template release when it has none) and the draft values of the request. This operation is read-only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## resolve_platform_component_configuration

> models::PlatformComponentConfigurationPreviewResponse resolve_platform_component_configuration(organization_id, cluster_id, component_key, platform_component_configuration_preview_request)
Resolve a platform component configuration

Resolves the fields and runtime requirements to display for a component using the cluster context and the values currently entered in the Console. This operation is read-only. Deprecated: use resolveClusterPlatformComponentConfiguration instead.

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

Selects a platform template and stores layer selections, component profile values, and customer-provided runtime inputs for the cluster. Deprecated: use updateClusterPlatformConfiguration instead.

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


## update_cluster_platform_configuration

> models::ClusterPlatformConfigurationResponse update_cluster_platform_configuration(cluster_id, cluster_platform_configuration_request)
Update the cluster platform configuration

Replaces the whole platform configuration of the cluster, its platform selection and its cluster inputs, after validating it against the template release. Saving does not deploy it. `\"<redacted>\"` is not a keep-existing value: never send it back.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |
**cluster_platform_configuration_request** | [**ClusterPlatformConfigurationRequest**](ClusterPlatformConfigurationRequest.md) |  | [required] |

### Return type

[**models::ClusterPlatformConfigurationResponse**](ClusterPlatformConfigurationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

