# ClusterFeatureKarpenterParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**spot_enabled** | **bool** | Deprecated: use the per-pool `spot_enabled` fields on `qovery_node_pools.default_override`, `qovery_node_pools.stable_override` and `qovery_node_pools.cronjob_override` instead.  On read, this is a derived value: it is recomputed on every write as the logical OR of the per-pool `spot_enabled` fields of the node pools present (the default and stable node pools, plus the cronjob node pool only while its `cronjob_override` block exists).  On write, it is only honoured for legacy clients: when none of the per-pool `spot_enabled` fields are present in the request, this value is applied to all node pools, including the stable one. As soon as the per-pool fields are present they take precedence and this value is ignored. | 
**disk_size_in_gib** | **i32** |  | 
**disk_iops** | Option<**i32**> | Unit is operation/seconds. The disk IOPS to be used for the node configuration | [optional]
**disk_throughput** | Option<**i32**> | Unit is in MB/s. The disk throughput to be used for the node configuration | [optional]
**default_service_architecture** | [**models::CpuArchitectureEnum**](CpuArchitectureEnum.md) |  | 
**qovery_node_pools** | [**models::KarpenterNodePool**](KarpenterNodePool.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


