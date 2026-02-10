# Cluster

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**organization** | [**models::ReferenceObject**](ReferenceObject.md) |  | 
**name** | **String** | name is case-insensitive | 
**description** | Option<**String**> |  | [optional]
**region** | **String** |  | 
**cloud_provider** | [**models::CloudVendorEnum**](CloudVendorEnum.md) |  | 
**min_running_nodes** | Option<**i32**> |  | [optional][default to 1]
**max_running_nodes** | Option<**i32**> |  | [optional][default to 1]
**disk_size** | Option<**i32**> | Unit is in GB. The disk size to be used for the node configuration | [optional][default to 20]
**disk_iops** | Option<**i32**> | Unit is operation/seconds. The disk IOPS to be used for the node configuration | [optional]
**disk_throughput** | Option<**i32**> | Unit is in MB/s. The disk thoughput to be used for the node configuration | [optional]
**instance_type** | Option<**String**> | the instance type to be used for this cluster. The list of values can be retrieved via the endpoint /{CloudProvider}/instanceType | [optional]
**kubernetes** | Option<[**models::KubernetesEnum**](KubernetesEnum.md)> |  | [optional]
**cpu** | Option<**i32**> | unit is millicores (m). 1000m = 1 cpu | [optional]
**memory** | Option<**i32**> | unit is MB. 1024 MB = 1GB | [optional]
**estimated_cloud_provider_cost** | Option<**i32**> | This is an estimation of the cost this cluster will represent on your cloud proider bill, based on your current configuration | [optional]
**status** | Option<[**models::ClusterStateEnum**](ClusterStateEnum.md)> |  | [optional]
**has_access** | Option<**bool**> |  | [optional]
**version** | Option<**String**> |  | [optional]
**is_default** | Option<**bool**> |  | [optional]
**is_demo** | Option<**bool**> | specific flag to indicate that this cluster is a demo one | [optional]
**production** | Option<**bool**> | specific flag to indicate that this cluster is a production one | [optional]
**ssh_keys** | Option<**Vec<String>**> | Indicate your public ssh_key to remotely connect to your EC2 instance. | [optional]
**features** | Option<[**Vec<models::ClusterFeatureResponse>**](ClusterFeatureResponse.md)> |  | [optional]
**deployment_status** | Option<[**models::ClusterDeploymentStatusEnum**](ClusterDeploymentStatusEnum.md)> |  | [optional]
**metrics_parameters** | Option<[**models::MetricsParameters**](MetricsParameters.md)> |  | [optional]
**infrastructure_outputs** | Option<[**models::InfrastructureOutputs**](InfrastructureOutputs.md)> |  | [optional]
**infrastructure_charts_parameters** | Option<[**models::ClusterInfrastructureChartsParameters**](ClusterInfrastructureChartsParameters.md)> |  | [optional]
**keda** | Option<[**models::ClusterKeda**](ClusterKeda.md)> |  | [optional]
**labels_groups** | Option<[**models::ClusterLabelsGroupList**](ClusterLabelsGroupList.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


