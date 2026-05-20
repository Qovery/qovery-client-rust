# ClusterStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** |  | 
**status** | [**models::ClusterStateEnum**](ClusterStateEnum.md) |  | 
**is_deployed** | **bool** |  | 
**next_k8s_available_version** | Option<**String**> |  | [optional]
**last_execution_id** | Option<**String**> |  | [optional]
**cluster_lock** | Option<[**models::ClusterLock**](ClusterLock.md)> |  | [optional]
**last_deployment_date** | Option<**String**> |  | [optional]
**reason** | [**models::DeploymentInfraReason**](DeploymentInfraReason.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


