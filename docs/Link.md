# Link

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | Option<**String**> |  | [optional]
**internal_port** | Option<**i32**> | The port from which the service is reachable from within the cluster | [optional]
**external_port** | Option<**i32**> | The port from which the service is reachable from externally (i.e: 443 for HTTPS) | [optional]
**is_qovery_domain** | Option<**bool**> | True if the domain is managed by Qovery, false if it belongs to the user | [optional]
**is_default** | Option<**bool**> | Indicate if the link is using the root of the domain and not one derivated from port i.e: p8080.zxxxx.jvm.worl      => is_default = false, is_qovery = true zxxxx.jvm.world           => is_default = true, is_qovery = true p8080-my-super-domain.com => is_default = false, is_qovery = false my-super-domain.com       => is_default = true, is_qovery = false  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


