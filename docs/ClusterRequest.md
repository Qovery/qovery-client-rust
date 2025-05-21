# ClusterRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | name is case-insensitive | 
**description** | Option<**String**> |  | [optional]
**region** | **String** |  | 
**cloud_provider** | [**models::CloudVendorEnum**](CloudVendorEnum.md) |  | 
**cloud_provider_credentials** | Option<[**models::ClusterCloudProviderInfoRequest**](ClusterCloudProviderInfoRequest.md)> |  | [optional]
**min_running_nodes** | Option<**i32**> |  | [optional][default to 1]
**max_running_nodes** | Option<**i32**> |  | [optional][default to 1]
**disk_size** | Option<**i32**> | Unit is in GB. The disk size to be used for the node configuration | [optional][default to 40]
**instance_type** | Option<**String**> | the instance type to be used for this cluster. The list of values can be retrieved via the endpoint /{CloudProvider}/instanceType | [optional]
**kubernetes** | Option<[**models::KubernetesEnum**](KubernetesEnum.md)> |  | [optional]
**production** | Option<**bool**> | specific flag to indicate that this cluster is a production one | [optional]
**ssh_keys** | Option<**Vec<String>**> | Indicate your public ssh_key to remotely connect to your EC2 instance. | [optional]
**features** | Option<[**Vec<models::ClusterRequestFeaturesInner>**](ClusterRequest_features_inner.md)> |  | [optional]
**metrics_parameters** | Option<[**models::MetricsParameters**](MetricsParameters.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


