# HelmPortResponseWithServiceSelectors

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**port_type** | **String** |  | 
**name** | Option<**String**> |  | [optional]
**internal_port** | **i32** | The listening port of your service. | 
**external_port** | Option<**i32**> | The exposed port for your service. This is optional. If not set a default port will be used. | [optional]
**namespace** | Option<**String**> |  | [optional]
**protocol** | [**models::HelmPortProtocolEnum**](HelmPortProtocolEnum.md) |  | 
**is_default** | Option<**bool**> | is the default port to use for domain | [optional]
**service_selectors** | [**Vec<models::KubernetesSelector>**](KubernetesSelector.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


