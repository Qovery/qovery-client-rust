# ApplicationAdvancedSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**deployment_period_custom_domain_check_enabled** | Option<**bool**> | disable custom domain check when deploying an application | [optional]
**deployment_period_termination_grace_period_seconds** | Option<**i32**> | define how long in seconds an application is supposed to be stopped gracefully | [optional]
**deployment_period_affinity_period_node_period_required** | Option<**std::collections::HashMap<String, String>**> | Set pod placement on specific Kubernetes nodes labels | [optional]
**deployment_period_antiaffinity_period_pod** | Option<**String**> | Define how you want pods affinity to behave: * `Preferred` allows, but does not require, pods of a given service are not co-located (or co-hosted) on a single node * `Requirred` ensures that the pods of a given service are not co-located (or co-hosted) on a single node (safer in term of availability but can be expensive depending on the number of replicas)  | [optional]
**deployment_period_lifecycle_period_post_start_exec_command** | Option<**Vec<String>**> | Allows you to run a command after the application is started. The command should be a shell command or script. | [optional]
**deployment_period_lifecycle_period_pre_stop_exec_command** | Option<**Vec<String>**> | Allows you to run a command before the application is stopped. The command should be a shell command or script. Qovery requires the sh shell by default and sets a sleep of 15 seconds to let Nginx update its config. Avoiding error codes returned during a rolling update. | [optional]
**deployment_period_update_strategy_period_type** | Option<**String**> | * `RollingUpdate` gracefully rollout new versions, and automatically rollback if the new version fails to start * `Recreate` stop all current versions and create new ones once all old ones have been shutdown  | [optional]
**deployment_period_update_strategy_period_rolling_update_period_max_unavailable_percent** | Option<**i32**> | Define the percentage of a maximum number of pods that can be unavailable during the update process | [optional]
**deployment_period_update_strategy_period_rolling_update_period_max_surge_percent** | Option<**i32**> | Define the percentage of the maximum number of pods that can be created over the desired number of pods | [optional]
**build_period_timeout_max_sec** | Option<**i32**> |  | [optional]
**build_period_cpu_max_in_milli** | Option<**i32**> | define the max cpu resources (in milli) | [optional]
**build_period_ram_max_in_gib** | Option<**i32**> | define the max ram resources (in gib) | [optional]
**build_period_disable_buildkit_cache** | Option<**bool**> | disable buildkit registry cache during build | [optional]
**network_period_ingress_period_proxy_body_size_mb** | Option<**i32**> |  | [optional]
**network_period_ingress_period_force_ssl_redirect** | Option<**bool**> | When using SSL offloading outside of cluster, you can enforce a redirect to HTTPS even when there is no TLS certificate available | [optional]
**network_period_ingress_period_enable_cors** | Option<**bool**> |  | [optional]
**network_period_ingress_period_cors_allow_origin** | Option<**String**> |  | [optional]
**network_period_ingress_period_cors_allow_methods** | Option<**String**> |  | [optional]
**network_period_ingress_period_cors_allow_headers** | Option<**String**> |  | [optional]
**network_period_ingress_period_proxy_buffer_size_kb** | Option<**i32**> | header buffer size used while reading response header from upstream | [optional]
**network_period_ingress_period_keepalive_time_seconds** | Option<**i32**> | Limits the maximum time (in seconds) during which requests can be processed through one keepalive connection | [optional]
**network_period_ingress_period_keepalive_timeout_seconds** | Option<**i32**> | Sets a timeout (in seconds) during which an idle keepalive connection to an upstream server will stay open. | [optional]
**network_period_ingress_period_send_timeout_seconds** | Option<**i32**> | Sets a timeout (in seconds) for transmitting a response to the client | [optional]
**network_period_ingress_period_proxy_connect_timeout_seconds** | Option<**i32**> | Sets a timeout (in seconds) for establishing a connection to a proxied server | [optional]
**network_period_ingress_period_proxy_send_timeout_seconds** | Option<**i32**> | Sets a timeout (in seconds) for transmitting a request to the proxied server | [optional]
**network_period_ingress_period_proxy_read_timeout_seconds** | Option<**i32**> | Sets a timeout (in seconds) for reading a response from the proxied server | [optional]
**network_period_ingress_period_proxy_buffering** | Option<**String**> | Allows to enable or disable nginx `proxy-request-buffering` | [optional]
**network_period_ingress_period_whitelist_source_range** | Option<**String**> | list of source ranges to allow access to ingress proxy.  This property can be used to whitelist source IP ranges for ingress proxy. The value is a comma separated list of CIDRs, e.g. 10.0.0.0/24,172.10.0.1 To allow all source ranges, set 0.0.0.0/0.  | [optional]
**network_period_ingress_period_denylist_source_range** | Option<**String**> | list of source ranges to deny access to ingress proxy.  This property can be used to blacklist source IP ranges for ingress proxy. The value is a comma separated list of CIDRs, e.g. 10.0.0.0/24,172.10.0.1  | [optional]
**network_period_ingress_period_basic_auth_env_var** | Option<**String**> | Set the name of an environment variable to use as a basic authentication (`login:crypted_password`) from `htpasswd` command.  | [optional]
**network_period_ingress_period_enable_sticky_session** | Option<**bool**> | Enable the load balancer to bind a user's session to a specific target. This ensures that all requests from the user during the session are sent to the same target  | [optional]
**network_period_ingress_period_grpc_send_timeout_seconds** | Option<**i32**> | Sets a timeout (in seconds) for transmitting a request to the grpc server | [optional]
**network_period_ingress_period_grpc_read_timeout_seconds** | Option<**i32**> | Sets a timeout (in seconds) for transmitting a request to the grpc server | [optional]
**network_period_ingress_period_extra_headers** | Option<**String**> | Allows to define response headers | [optional]
**hpa_period_cpu_period_average_utilization_percent** | Option<**i32**> | Percentage value of cpu usage at which point pods should scale up. | [optional]
**hpa_period_memory_period_average_utilization_percent** | Option<**i32**> | Percentage value of memory usage at which point pods should scale up. | [optional]
**security_period_service_account_name** | Option<**String**> | Allows you to set an existing Kubernetes service account name  | [optional]
**security_period_automount_service_account_token** | Option<**bool**> | Automount Kubernetes service account token to have access to Kubernetes API from pods  | [optional]
**security_period_read_only_root_filesystem** | Option<**bool**> | Mounts the container's root filesystem as read-only  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


