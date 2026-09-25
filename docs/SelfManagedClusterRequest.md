# SelfManagedClusterRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**production** | Option<**bool**> |  | [optional][default to false]
**provider** | **Provider** |  (enum: AWS) | 
**region** | **String** |  | 
**credentials** | [**models::SelfManagedClusterCredentials**](SelfManagedClusterCredentials.md) |  | 
**platform** | [**models::PlatformSelection**](PlatformSelection.md) |  | 
**cluster_inputs** | Option<**std::collections::HashMap<String, std::collections::HashMap<String, String>>**> | String values keyed first by component key and then by input key | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


