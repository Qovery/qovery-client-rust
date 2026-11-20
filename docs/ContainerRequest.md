# ContainerRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**storage** | Option<[**Vec<models::ServiceStorageRequestStorageInner>**](ServiceStorageRequestStorageInner.md)> |  | [optional]
**ports** | Option<[**Vec<models::ServicePortRequestPortsInner>**](ServicePortRequestPortsInner.md)> |  | [optional]
**name** | **String** | name is case insensitive | 
**description** | Option<**String**> | give a description to this container | [optional]
**registry_id** | Option<**String**> | id of the linked registry. Required on create unless `source` is sent. See `source` for the rules on update.  | [optional]
**image_name** | Option<**String**> | The image name pattern differs according to chosen container registry provider:   * `ECR`: `repository` * `SCALEWAY_CR`: `namespace/image` * `DOCKER_HUB`: `image` or `repository/image` * `PUBLIC_ECR`: `registry_alias/repository`  Required on create unless `source` is sent. See `source` for the rules on update.  | [optional]
**tag** | Option<**String**> | tag of the image container. Required on create unless `source` is sent. See `source` for the rules on update.  | [optional]
**source** | Option<[**models::ContainerSourceRequest**](ContainerSourceRequest.md)> |  | [optional]
**arguments** | Option<**Vec<String>**> |  | [optional]
**entrypoint** | Option<**String**> | optional entrypoint when launching container | [optional]
**cpu** | Option<**i32**> | unit is millicores (m). 1000m = 1 cpu | [optional][default to 500]
**memory** | Option<**i32**> | unit is MB. 1024 MB = 1GB | [optional][default to 512]
**gpu** | Option<**i32**> |  | [optional][default to 0]
**ephemeral_storage_in_gib** | Option<**i32**> | Ephemeral storage of the service in GiB. When omitted, the platform default is used. | [optional]
**cpu_architecture** | Option<[**models::CpuArchitectureEnum**](CpuArchitectureEnum.md)> | CPU architecture to run this service on. If null, the cluster default architecture is used. | [optional]
**min_running_instances** | Option<**i32**> | Minimum number of instances running. This resource auto-scale based on the CPU and Memory consumption. Note: 0 means that there is no container running.  | [optional][default to 1]
**max_running_instances** | Option<**i32**> | Maximum number of instances running. This resource auto-scale based on the CPU and Memory consumption. Note: -1 means that there is no limit.  | [optional][default to 1]
**healthchecks** | [**models::Healthcheck**](Healthcheck.md) |  | 
**auto_preview** | Option<**bool**> | Indicates if the 'environment preview option' is enabled for this container.   If enabled, a preview environment will be automatically cloned when `/preview` endpoint is called.   If not specified, it takes the value of the `auto_preview` property from the associated environment.  On a container with a Git repository source, it must be false or not specified: `true` returns 400. When not specified on a create from, or a switch to, `git_repository`, it is set to false.  | [optional]
**auto_deploy** | Option<**bool**> | Specify if the container will be automatically updated after receiving a new image tag.  The new image tag shall be communicated via the \"Auto Deploy container\" endpoint https://api-doc.qovery.com/#tag/Containers/operation/autoDeployContainerEnvironments  On a container with a Git repository source, it must be false or not specified: `true` returns 400. When not specified on a create from, or a switch to, `git_repository`, it is set to false.  | [optional]
**annotations_groups** | Option<[**Vec<models::ServiceAnnotationRequest>**](ServiceAnnotationRequest.md)> |  | [optional]
**labels_groups** | Option<[**Vec<models::ServiceLabelRequest>**](ServiceLabelRequest.md)> |  | [optional]
**icon_uri** | Option<**String**> | Icon URI representing the container. | [optional]
**autoscaling** | Option<[**models::KedaAutoscalingRequest**](KedaAutoscalingRequest.md)> |  | [optional]
**build_settings** | Option<[**models::BuildSettings**](BuildSettings.md)> | Build settings of a container with a Git repository source (`source.git_repository`). They apply only to the build of its image. - When sent, it replaces all stored values: omitted properties take their defaults, they are not merged. - On update, when not sent, the stored values are kept. - On a create from, or a switch to, `git_repository` without it, the defaults of a new application are used. - Sending it when the container is created with, keeps, or switches to, an image source returns 400.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


