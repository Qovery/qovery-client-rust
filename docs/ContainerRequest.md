# ContainerRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**storage** | Option<[**Vec<models::ServiceStorageRequestStorageInner>**](ServiceStorageRequest_storage_inner.md)> |  | [optional]
**ports** | Option<[**Vec<models::ServicePortRequestPortsInner>**](ServicePortRequest_ports_inner.md)> |  | [optional]
**name** | **String** | name is case insensitive | 
**description** | Option<**String**> | give a description to this container | [optional]
**registry_id** | **String** | id of the linked registry | 
**image_name** | **String** | The image name pattern differs according to chosen container registry provider:   * `ECR`: `repository` * `SCALEWAY_CR`: `namespace/image` * `DOCKER_HUB`: `image` or `repository/image` * `PUBLIC_ECR`: `registry_alias/repository`  | 
**tag** | **String** | tag of the image container | 
**arguments** | Option<**Vec<String>**> |  | [optional]
**entrypoint** | Option<**String**> | optional entrypoint when launching container | [optional]
**cpu** | Option<**i32**> | unit is millicores (m). 1000m = 1 cpu | [optional][default to 500]
**memory** | Option<**i32**> | unit is MB. 1024 MB = 1GB | [optional][default to 512]
**gpu** | Option<**i32**> |  | [optional][default to 0]
**min_running_instances** | Option<**i32**> | Minimum number of instances running. This resource auto-scale based on the CPU and Memory consumption. Note: 0 means that there is no container running.  | [optional][default to 1]
**max_running_instances** | Option<**i32**> | Maximum number of instances running. This resource auto-scale based on the CPU and Memory consumption. Note: -1 means that there is no limit.  | [optional][default to 1]
**healthchecks** | [**models::Healthcheck**](Healthcheck.md) |  | 
**auto_preview** | Option<**bool**> | Indicates if the 'environment preview option' is enabled for this container.   If enabled, a preview environment will be automatically cloned when `/preview` endpoint is called.   If not specified, it takes the value of the `auto_preview` property from the associated environment.  | [optional]
**auto_deploy** | Option<**bool**> | Specify if the container will be automatically updated after receiving a new image tag.  The new image tag shall be communicated via the \"Auto Deploy container\" endpoint https://api-doc.qovery.com/#tag/Containers/operation/autoDeployContainerEnvironments  | [optional]
**annotations_groups** | Option<[**Vec<models::ServiceAnnotationRequest>**](ServiceAnnotationRequest.md)> |  | [optional]
**labels_groups** | Option<[**Vec<models::ServiceLabelRequest>**](ServiceLabelRequest.md)> |  | [optional]
**icon_uri** | Option<**String**> | Icon URI representing the container. | [optional]
**autoscaling** | Option<[**models::KedaAutoscalingRequest**](KedaAutoscalingRequest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


