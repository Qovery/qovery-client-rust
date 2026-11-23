# CustomDomainErrorResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | [**models::CustomDomainErrorCodeEnum**](CustomDomainErrorCodeEnum.md) |  | 
**field** | **String** | The request field to blame, named as on the wire, such as `domain`, `path`, `port_id`, `generate_certificate` or `use_cdn`. | 
**path** | Option<**String**> | The path in conflict. Null when the error is not about a path. | 
**service_id** | **uuid::Uuid** | Id of the service of the request that cannot hold the domain. | 
**conflicting_service** | Option<[**models::CustomDomainServiceResponse**](CustomDomainServiceResponse.md)> | The service of the same environment the domain conflicts with. Null when the conflict is with another environment, which the caller may not have access to. | 
**message** | **String** | A message to show to the user. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


