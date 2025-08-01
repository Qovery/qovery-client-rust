# Link

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**service_id** | [**uuid::Uuid**](uuid::Uuid.md) | ID of the associated service | 
**service_type** | [**models::ServiceTypeEnum**](ServiceTypeEnum.md) |  | 
**url** | **String** | URL to access the service | 
**internal_port** | **i32** | The port from which the service is reachable from within the cluster | 
**external_port** | **i32** | The port from which the service is reachable from externally (i.e: 443 for HTTPS) | 
**is_qovery_domain** | **bool** | True if the domain is managed by Qovery, false if it belongs to the user | 
**is_default** | **bool** | Indicate if the link is using the root of the domain and not one derivated from port i.e: p8080.zxxxx.jvm.worl      => is_default = false, is_qovery = true zxxxx.jvm.world           => is_default = true, is_qovery = true p8080-my-super-domain.com => is_default = false, is_qovery = false my-super-domain.com       => is_default = true, is_qovery = false  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


