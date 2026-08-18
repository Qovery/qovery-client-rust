# ClusterOperatorFleetInventoryResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** |  | 
**cluster_id** | **uuid::Uuid** |  | 
**cluster_name** | **String** |  | 
**cluster_kind** | [**models::SelfManagedClusterKind**](SelfManagedClusterKind.md) |  | 
**attached** | **bool** | Whether the cluster is explicitly routed through the Operator execution path. | 
**connected** | **bool** | Whether the last heartbeat is within the Operator presence window. | 
**last_heartbeat** | Option<**String**> |  | [optional]
**desired_image_version** | Option<**String**> |  | [optional]
**reported_image_version** | Option<**String**> |  | [optional]
**desired_chart_version** | Option<**String**> |  | [optional]
**reported_chart_version** | Option<**String**> |  | [optional]
**status** | [**models::ClusterOperatorFleetStatus**](ClusterOperatorFleetStatus.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


