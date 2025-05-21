# ClusterAdvancedSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aws_period_cloudwatch_period_eks_logs_retention_days** | Option<**i32**> | Set the number of retention days for EKS Cloudwatch logs | [optional]
**aws_period_vpc_period_enable_s3_flow_logs** | Option<**bool**> | Enable flow logs for on the VPC and store them in an S3 bucket | [optional]
**aws_period_vpc_period_flow_logs_retention_days** | Option<**i32**> | Set the number of retention days for flow logs. Disable with value \"0\" | [optional]
**loki_period_log_retention_in_week** | Option<**i32**> | For how long in week loki is going to keep logs of your applications | [optional]
**registry_period_image_retention_time** | Option<**i32**> | Configure the number of seconds before cleaning images in the registry | [optional]
**cloud_provider_period_container_registry_period_tags** | Option<**std::collections::HashMap<String, String>**> | Add additional tags on the cluster dedicated registry | [optional]
**aws_period_eks_period_enable_alb_controller** | Option<**bool**> | Enable the AWS ALB controller to manage the load balancer for the cluster. Note: Changing this feature will create a 10 min max downtime on your application's public access (time to delete, replace and propagate DNS of the new load balancer) and will requiere to update all services with TCP/UDP open ports. | [optional][default to true]
**load_balancer_period_size** | Option<**String**> | Select the size of the main load_balancer (only effective for Scaleway) | [optional]
**database_period_postgresql_period_deny_any_access** | Option<**bool**> | Deny public access to any PostgreSQL database | [optional]
**database_period_postgresql_period_allowed_cidrs** | Option<**Vec<String>**> | List of CIDRs allowed to access the PostgreSQL database | [optional]
**database_period_mysql_period_deny_any_access** | Option<**bool**> | Deny public access to any MySql database | [optional]
**database_period_mysql_period_allowed_cidrs** | Option<**Vec<String>**> | List of CIDRs allowed to access the MySql database | [optional]
**database_period_mongodb_period_deny_any_access** | Option<**bool**> | Deny public access to any MongoDB/DocumentDB database | [optional]
**database_period_mongodb_period_allowed_cidrs** | Option<**Vec<String>**> | List of CIDRs allowed to access the MongoDB/DocumentDB database | [optional]
**database_period_redis_period_deny_any_access** | Option<**bool**> | Deny public access to any Redis database | [optional]
**database_period_redis_period_allowed_cidrs** | Option<**Vec<String>**> | List of CIDRs allowed to access the Redis database | [optional]
**aws_period_iam_period_admin_group** | Option<**String**> | AWS IAM group name with cluster access | [optional]
**aws_period_eks_period_ec2_period_metadata_imds** | Option<**String**> | Specify the [IMDS](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html) version you want to use:   * `required`: IMDS V2 only   * `optional`: IMDS V1 + V2  | [optional]
**pleco_period_resources_ttl** | Option<**i32**> |  | [optional]
**registry_period_mirroring_mode** | Option<[**models::RegistryMirroringModeEnum**](RegistryMirroringModeEnum.md)> |  | [optional]
**nginx_period_vcpu_period_request_in_milli_cpu** | Option<**i32**> | vcpu request in millicores | [optional]
**nginx_period_vcpu_period_limit_in_milli_cpu** | Option<**i32**> | vcpu limit in millicores | [optional]
**nginx_period_memory_period_request_in_mib** | Option<**i32**> | memory request in MiB | [optional]
**nginx_period_memory_period_limit_in_mib** | Option<**i32**> | memory limit in MiB | [optional]
**nginx_period_hpa_period_cpu_utilization_percentage_threshold** | Option<**i32**> | hpa cpu threshold in percentage | [optional]
**nginx_period_hpa_period_min_number_instances** | Option<**i32**> | hpa minimum number of instances | [optional]
**nginx_period_hpa_period_max_number_instances** | Option<**i32**> | hpa maximum number of instances | [optional]
**storageclass_period_fast_ssd** | Option<**String**> | storage class name to use to provision pvc | [optional]
**qovery_period_static_ip_mode** | Option<**bool**> | To limit public access from the internet to your Kubernetes cluster endpoint. You can define whitelisted CIDR in k8s.api.allowed_public_access_cidrs. | [optional]
**k8s_period_api_period_allowed_public_access_cidrs** | Option<**Vec<String>**> | Set custom sources to public access endpoint. Use CIDR notation to specify an IP address range  (for example, ['203.0.113.5/32','203.0.100/32']) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


