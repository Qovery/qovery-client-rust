# KarpenterDefaultNodePoolOverride

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**limits** | Option<[**models::KarpenterNodePoolLimits**](KarpenterNodePoolLimits.md)> |  | [optional]
**spot_enabled** | Option<**bool**> | Whether this node pool runs on spot instances. `null` or absent means the pool inherits the deprecated top-level `spot_enabled`: on write that value applies to this pool, on read only a deviating value is surfaced. `default_override` is omitted from a response when it would carry nothing else. | [optional]
**consolidate_after** | Option<**String**> | Time to wait before consolidating empty or underutilized nodes (e.g., 1m, 10m, 1h). Maximum: 24h | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


