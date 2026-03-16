# ClusterFeatureKarpenterParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**spot_enabled** | **bool** |  | 
**disk_size_in_gib** | **i32** |  | 
**disk_iops** | Option<**i32**> | Unit is operation/seconds. The disk IOPS to be used for the node configuration | [optional]
**disk_throughput** | Option<**i32**> | Unit is in MB/s. The disk throughput to be used for the node configuration | [optional]
**default_service_architecture** | [**models::CpuArchitectureEnum**](CpuArchitectureEnum.md) |  | 
**qovery_node_pools** | [**models::KarpenterNodePool**](KarpenterNodePool.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


