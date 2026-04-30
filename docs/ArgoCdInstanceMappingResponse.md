# ArgoCdInstanceMappingResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent_cluster_id** | **uuid::Uuid** | ID of the Qovery cluster where the ArgoCD instance is running | 
**agent_cluster_name** | **String** | Display name of the Qovery cluster where the ArgoCD instance is running | 
**agent_cluster_cloud_provider** | [**models::CloudVendorEnum**](CloudVendorEnum.md) |  | 
**credentials_id** | **uuid::Uuid** | ID of the stored ArgoCD credentials for this instance | 
**argocd_url** | **String** | URL of the ArgoCD instance | 
**status** | [**models::ArgoCdConnectionStatusEnum**](ArgoCdConnectionStatusEnum.md) |  | 
**last_checked_at** | **String** | Timestamp of the last configuration update (will use last connectivity check in future) | 
**linked_clusters** | [**Vec<models::ArgoCdLinkedClusterDetails>**](ArgoCdLinkedClusterDetails.md) | ArgoCD clusters detected and mapped to a Qovery cluster | 
**unlinked_clusters** | [**Vec<models::ArgoCdUnlinkedClusterDetails>**](ArgoCdUnlinkedClusterDetails.md) | ArgoCD clusters detected but mapping is missing | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


