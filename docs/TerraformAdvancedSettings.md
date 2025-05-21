# TerraformAdvancedSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**build_period_timeout_max_sec** | Option<**i32**> | define the max timeout for the build | [optional]
**build_period_cpu_max_in_milli** | Option<**i32**> | define the max cpu resources (in milli) | [optional]
**build_period_ram_max_in_gib** | Option<**i32**> | define the max ram resources (in gib) | [optional]
**build_period_ephemeral_storage_in_gib** | Option<**i32**> |  | [optional]
**deployment_period_termination_grace_period_seconds** | Option<**i32**> | define how long in seconds an application is supposed to be stopped gracefully | [optional]
**deployment_period_affinity_period_node_period_required** | Option<**std::collections::HashMap<String, String>**> | Set pod placement on specific Kubernetes nodes labels | [optional]
**security_period_service_account_name** | Option<**String**> | Allows you to set an existing Kubernetes service account name  | [optional]
**security_period_read_only_root_filesystem** | Option<**bool**> | Mounts the container's root filesystem as read-only  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


