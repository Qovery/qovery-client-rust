# HelmResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**environment** | [**models::ReferenceObject**](ReferenceObject.md) |  | 
**name** | **String** | name is case insensitive | 
**description** | Option<**String**> |  | [optional]
**timeout_sec** | Option<**i32**> | Maximum number of seconds allowed for helm to run before killing it and mark it as failed  | [optional][default to 600]
**auto_preview** | **bool** | Indicates if the 'environment preview option' is enabled.   If enabled, a preview environment will be automatically cloned when `/preview` endpoint is called.   If not specified, it takes the value of the `auto_preview` property from the associated environment.  | 
**auto_deploy** | **bool** | Specify if the service will be automatically updated after receiving a new image tag or a new commit according to the source type.  | 
**ports** | Option<[**Vec<models::HelmResponseAllOfPorts>**](HelmResponse_allOf_ports.md)> |  | [optional]
**source** | [**models::HelmResponseAllOfSource**](HelmResponse_allOf_source.md) |  | 
**arguments** | **Vec<String>** | The extra arguments to pass to helm | 
**allow_cluster_wide_resources** | **bool** | If we should allow the chart to deploy object outside his specified namespace. Setting this flag to true, requires special rights  | [default to false]
**values_override** | [**models::HelmResponseAllOfValuesOverride**](HelmResponse_allOf_values_override.md) |  | 
**icon_uri** | **String** | Icon URI representing the helm service. | 
**service_type** | [**models::ServiceTypeEnum**](ServiceTypeEnum.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


