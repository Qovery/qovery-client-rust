# ServicePort

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**name** | Option<**String**> |  | [optional]
**internal_port** | **i32** | The listening port of your service. | 
**external_port** | Option<**i32**> | The exposed port for your service. This is optional. If not set a default port will be used. | [optional]
**publicly_accessible** | **bool** | Expose the port to the world | 
**is_default** | Option<**bool**> | is the default port to use for domain | [optional]
**protocol** | [**models::PortProtocolEnum**](PortProtocolEnum.md) |  | 
**public_path** | Option<**String**> | Indicate the path or regex that must match for traffic to be accepted on your service i.e: /api/ will only accept http calls that start with /api/  Only valid for publicly_accessible HTTP or GRPC ports. | [optional]
**public_path_rewrite** | Option<**String**> | Indicate the new path that will be used to reach your service after replacement i.e: public_path -> /(.*)  public_path_rewrite -> /api/$1 will append /api/ on all externaly requested url when reaching the service  external/use url -> example.com/foobar  -> url seen by the service -> example.com/api/foobar Only valid for publicly_accessible HTTP or GRPC ports. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


