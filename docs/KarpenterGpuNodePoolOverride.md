# KarpenterGpuNodePoolOverride

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**consolidation** | Option<[**models::KarpenterNodePoolConsolidation**](KarpenterNodePoolConsolidation.md)> |  | [optional]
**limits** | Option<[**models::KarpenterNodePoolLimits**](KarpenterNodePoolLimits.md)> |  | [optional]
**requirements** | Option<[**Vec<models::KarpenterNodePoolRequirement>**](KarpenterNodePoolRequirement.md)> |  | [optional]
**disk_size_in_gib** | Option<**i32**> |  | [optional][default to 100]
**disk_iops** | Option<**i32**> | Unit is operation/seconds. The disk IOPS to be used for the GPU node pool configuration | [optional]
**disk_throughput** | Option<**i32**> | Unit is in MB/s. The disk throughput to be used for the GPU node pool configuration | [optional]
**spot_enabled** | Option<**bool**> |  | [optional][default to false]
**consolidate_after** | Option<**String**> | Time to wait before consolidating empty or underutilized nodes (e.g., 1m, 10m, 1h). Maximum: 24h | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


