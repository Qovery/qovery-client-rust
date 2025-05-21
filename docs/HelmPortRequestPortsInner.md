# HelmPortRequestPortsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**internal_port** | **i32** | The listening port of your service. | 
**external_port** | Option<**i32**> | The exposed port for your service. This is optional. If not set a default port will be used. | [optional]
**namespace** | Option<**String**> |  | [optional]
**protocol** | Option<[**models::HelmPortProtocolEnum**](HelmPortProtocolEnum.md)> |  | [optional]
**is_default** | Option<**bool**> | is the default port to use for domain | [optional]
**service_selectors** | Option<[**Vec<models::KubernetesSelector>**](KubernetesSelector.md)> |  | [optional]
**service_name** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


