# HelmAdvancedSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**deployment_period_custom_domain_check_enabled** | Option<**bool**> | disable custom domain check when deploying a helm | [optional]
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
**network_period_ingress_period_proxy_buffering** | Option<**String**> | Allows to enable or disable nginx `proxy-buffering` | [optional]
**network_period_ingress_period_proxy_request_buffering** | Option<**String**> | Allows to enable or disable nginx `proxy-request-buffering` | [optional]
**network_period_ingress_period_grpc_send_timeout_seconds** | Option<**i32**> | Sets a timeout (in seconds) for transmitting a request to the grpc server | [optional]
**network_period_ingress_period_grpc_read_timeout_seconds** | Option<**i32**> | Sets a timeout (in seconds) for transmitting a request to the grpc server | [optional]
**network_period_ingress_period_whitelist_source_range** | Option<**String**> | list of source ranges to allow access to ingress proxy.  This property can be used to whitelist source IP ranges for ingress proxy. The value is a comma separated list of CIDRs, e.g. 10.0.0.0/24,172.10.0.1 To allow all source ranges, set 0.0.0.0/0.  | [optional]
**network_period_ingress_period_denylist_source_range** | Option<**String**> | list of source ranges to deny access to ingress proxy.  This property can be used to blacklist source IP ranges for ingress proxy. The value is a comma separated list of CIDRs, e.g. 10.0.0.0/24,172.10.0.1  | [optional]
**network_period_ingress_period_extra_headers** | Option<**String**> | Allows to define response headers | [optional]
**network_period_ingress_period_basic_auth_env_var** | Option<**String**> | Set the name of an environment variable to use as a basic authentication (`login:crypted_password`) from `htpasswd` command. You can add multiples comma separated values.  | [optional]
**network_period_ingress_period_enable_sticky_session** | Option<**bool**> | Enable the load balancer to bind a user's session to a specific target. This ensures that all requests from the user during the session are sent to the same target  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


