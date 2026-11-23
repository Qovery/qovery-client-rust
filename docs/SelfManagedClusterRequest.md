# SelfManagedClusterRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**production** | Option<**bool**> |  | [optional][default to false]
**provider** | **Provider** | Cloud provider of the cluster. The cluster kind is EKS_SELF_MANAGED on AWS, GKE_SELF_MANAGED on GCP and SCW_SELF_MANAGED on SCW. ON_PREMISE, HETZNER, OVH, CIVO, ORACLE and IBM are the on-premise providers, for any other Kubernetes: the cluster kind is SELF_MANAGED. They are not generally available yet and answer 400 outside a short list of Qovery test organizations. (enum: AWS, GCP, SCW, ON_PREMISE, HETZNER, OVH, CIVO, ORACLE, IBM) | 
**region** | **String** | Region of the provider known to Qovery; a zone such as `fr-par-1` on Scaleway; `on-premise` for the on-premise providers. | 
**credentials** | [**models::SelfManagedClusterCredentials**](SelfManagedClusterCredentials.md) |  | 
**platform** | [**models::SelfManagedClusterPlatformSelection**](SelfManagedClusterPlatformSelection.md) |  | 
**cluster_inputs** | Option<**std::collections::HashMap<String, std::collections::HashMap<String, String>>**> | String values keyed first by component key and then by input key | [optional]
**registry** | Option<[**models::SelfManagedClusterRegistryRequest**](SelfManagedClusterRegistryRequest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


