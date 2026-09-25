# SelfManagedClusterResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | 
**organization_id** | **uuid::Uuid** |  | 
**name** | **String** |  | 
**production** | **bool** |  | 
**provider** | [**models::CloudVendorEnum**](CloudVendorEnum.md) |  | 
**region** | **String** |  | 
**created_at** | **String** |  | 
**credentials** | [**models::SelfManagedClusterCredentials**](SelfManagedClusterCredentials.md) |  | 
**registry** | [**models::SelfManagedClusterRegistryResponse**](SelfManagedClusterRegistryResponse.md) |  | 
**platform** | [**models::PlatformSelection**](PlatformSelection.md) |  | 
**cluster_inputs** | **std::collections::HashMap<String, std::collections::HashMap<String, String>>** | String values keyed first by component key and then by input key | 
**layers** | [**Vec<models::ClusterPlatformBindingLayerResponse>**](ClusterPlatformBindingLayerResponse.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


