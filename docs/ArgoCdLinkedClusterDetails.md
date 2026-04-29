# ArgoCdLinkedClusterDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**argocd_cluster_url** | **String** | ArgoCD destination cluster URL | 
**argocd_cluster_name** | **String** | ArgoCD cluster name (currently same as URL; will use ArgoCD alias in future) | 
**qovery_cluster_id** | **uuid::Uuid** | ID of the Qovery cluster this destination is mapped to | 
**qovery_cluster_name** | **String** | Display name of the mapped Qovery cluster | 
**qovery_cluster_cloud_provider** | [**models::CloudVendorEnum**](CloudVendorEnum.md) |  | 
**qovery_cluster_type** | [**models::KubernetesEnum**](KubernetesEnum.md) |  | 
**applications_count** | **i32** | Number of ArgoCD applications targeting this destination | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


