# HelmRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ports** | Option<[**Vec<models::HelmPortRequestPortsInner>**](HelmPortRequest_ports_inner.md)> |  | [optional]
**name** | **String** | name is case insensitive | 
**description** | Option<**String**> |  | [optional]
**timeout_sec** | Option<**i32**> | Maximum number of seconds allowed for helm to run before killing it and mark it as failed  | [optional][default to 600]
**auto_preview** | Option<**bool**> | Indicates if the 'environment preview option' is enabled.   If enabled, a preview environment will be automatically cloned when `/preview` endpoint is called or when a new commit is updated. If not specified, it takes the value of the `auto_preview` property from the associated environment.  | [optional]
**auto_deploy** | **bool** | Specify if the helm will be automatically updated after receiving a new image tag or a new commit according to the source type.  | 
**source** | [**models::HelmRequestAllOfSource**](HelmRequest_allOf_source.md) |  | 
**arguments** | **Vec<String>** | The extra arguments to pass to helm | 
**allow_cluster_wide_resources** | Option<**bool**> | If we should allow the chart to deploy object outside his specified namespace. Setting this flag to true, requires special rights  | [optional][default to false]
**values_override** | [**models::HelmRequestAllOfValuesOverride**](HelmRequest_allOf_values_override.md) |  | 
**icon_uri** | Option<**String**> | Icon URI representing the helm service. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


