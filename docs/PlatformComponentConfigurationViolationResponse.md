# PlatformComponentConfigurationViolationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **String** | Stable machine-readable violation code. | 
**field_path** | **String** | Path of the offending field. Convention: profile configuration fields use the bare field key (e.g. `storage`); cluster-provided inputs are prefixed with `clusterInputs.` (e.g. `clusterInputs.s3BucketName`). Consumers match violations to form fields using these exact paths. | 
**message** | **String** | Customer-facing message describing how to fix the violation. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


