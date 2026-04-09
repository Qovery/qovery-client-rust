# ClusterInfrastructureEksAnywhereBackupParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<**bool**> | Enable or disable EKS Anywhere cluster backup. | [optional][default to true]
**timeout_seconds** | Option<**i64**> | Timeout in seconds for backup operations. | [optional][default to 300]
**certs_secret_name** | Option<**String**> | Optional Kubernetes secret name holding etcd certificates. | [optional]
**s3** | [**models::ClusterInfrastructureEksAnywhereBackupS3Parameters**](ClusterInfrastructureEksAnywhereBackupS3Parameters.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


