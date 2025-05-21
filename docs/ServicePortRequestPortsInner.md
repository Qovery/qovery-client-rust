# ServicePortRequestPortsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**internal_port** | **i32** | The listening port of your service. | 
**external_port** | Option<**i32**> | The exposed port for your service. This is optional. If not set a default port will be used. | [optional]
**publicly_accessible** | **bool** | Expose the port to the world | 
**is_default** | Option<**bool**> | is the default port to use for domain | [optional]
**protocol** | Option<[**models::PortProtocolEnum**](PortProtocolEnum.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


