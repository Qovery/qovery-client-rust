# JobAdvancedSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**build_period_timeout_max_sec** | Option<**i32**> | define the max timeout for the build | [optional]
**build_period_cpu_max_in_milli** | Option<**i32**> | define the max cpu resources (in milli) | [optional]
**build_period_ram_max_in_gib** | Option<**i32**> | define the max ram resources (in gib) | [optional]
**deployment_period_termination_grace_period_seconds** | Option<**i32**> | define how long in seconds an application is supposed to be stopped gracefully | [optional]
**deployment_period_affinity_period_node_period_required** | Option<**std::collections::HashMap<String, String>**> | Set pod placement on specific Kubernetes nodes labels | [optional]
**job_period_delete_ttl_seconds_after_finished** | Option<**i32**> |  | [optional]
**cronjob_period_concurrency_policy** | Option<**String**> |  | [optional]
**cronjob_period_failed_jobs_history_limit** | Option<**i32**> |  | [optional]
**cronjob_period_success_jobs_history_limit** | Option<**i32**> |  | [optional]
**security_period_service_account_name** | Option<**String**> | Allows you to set an existing Kubernetes service account name  | [optional]
**security_period_automount_service_account_token** | Option<**bool**> | Automount Kubernetes service account token to have access to Kubernetes API from pods  | [optional]
**security_period_read_only_root_filesystem** | Option<**bool**> | Mounts the container's root filesystem as read-only  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


