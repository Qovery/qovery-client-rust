# ContainerResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**storage** | Option<[**Vec<models::ServiceStorageStorageInner>**](ServiceStorage_storage_inner.md)> |  | [optional]
**image_name** | **String** | The image name pattern differs according to chosen container registry provider: * `ECR`: `repository` * `SCALEWAY_CR`: `namespace/image` * `DOCKER_HUB`: `image` or `repository/image` * `PUBLIC_ECR`: `registry_alias/repository`  | 
**tag** | **String** | tag of the image container | 
**registry_id** | Option<**String**> | tag of the image container | [optional]
**registry** | [**models::ContainerRegistryProviderDetailsResponse**](ContainerRegistryProviderDetailsResponse.md) |  | 
**environment** | [**models::ReferenceObject**](ReferenceObject.md) |  | 
**maximum_cpu** | **i32** | Maximum cpu that can be allocated to the container based on organization cluster configuration. unit is millicores (m). 1000m = 1 cpu | 
**maximum_memory** | **i32** | Maximum memory that can be allocated to the container based on organization cluster configuration. unit is MB. 1024 MB = 1GB | 
**maximum_gpu** | **i32** | Maximum memory that can be allocated to the container based on organization cluster configuration. unit is MB. 1024 MB = 1GB | [default to 0]
**name** | **String** | name is case insensitive | 
**description** | Option<**String**> | give a description to this container | [optional]
**arguments** | Option<**Vec<String>**> |  | [optional]
**entrypoint** | Option<**String**> | optional entrypoint when launching container | [optional]
**cpu** | **i32** | unit is millicores (m). 1000m = 1 cpu | 
**memory** | **i32** | unit is MB. 1024 MB = 1GB | 
**gpu** | **i32** |  | [default to 0]
**min_running_instances** | **i32** | Minimum number of instances running. This resource auto-scale based on the CPU and Memory consumption. Note: 0 means that there is no container running.  | [default to 1]
**max_running_instances** | **i32** | Maximum number of instances running. This resource auto-scale based on the CPU and Memory consumption. Note: -1 means that there is no limit.  | [default to 1]
**healthchecks** | [**models::Healthcheck**](Healthcheck.md) |  | 
**auto_preview** | **bool** | Indicates if the 'environment preview option' is enabled for this container.   If enabled, a preview environment will be automatically cloned when `/preview` endpoint is called.   If not specified, it takes the value of the `auto_preview` property from the associated environment.  | 
**ports** | Option<[**Vec<models::ServicePort>**](ServicePort.md)> |  | [optional]
**auto_deploy** | Option<**bool**> | Specify if the container will be automatically updated after receiving a new image tag.  The new image tag shall be communicated via the \"Auto Deploy container\" endpoint https://api-doc.qovery.com/#tag/Containers/operation/autoDeployContainerEnvironments  | [optional]
**annotations_groups** | Option<[**Vec<models::OrganizationAnnotationsGroupResponse>**](OrganizationAnnotationsGroupResponse.md)> |  | [optional]
**labels_groups** | Option<[**Vec<models::OrganizationLabelsGroupResponse>**](OrganizationLabelsGroupResponse.md)> |  | [optional]
**icon_uri** | **String** | Icon URI representing the container. | 
**service_type** | [**models::ServiceTypeEnum**](ServiceTypeEnum.md) |  | 
**autoscaling** | Option<[**models::KedaAutoscalingResponse**](KedaAutoscalingResponse.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


