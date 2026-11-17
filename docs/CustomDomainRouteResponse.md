# CustomDomainRouteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Id of the route. | 
**custom_domain_id** | **uuid::Uuid** | Id of this domain in the custom domain API of the service (`/application|container|helm/{id}/customDomain/{customDomainId}`). | 
**service** | [**models::CustomDomainServiceResponse**](CustomDomainServiceResponse.md) |  | 
**port** | Option<[**models::CustomDomainPortResponse**](CustomDomainPortResponse.md)> |  | 
**path** | **String** | The path of the requests this route serves. | 
**path_rewrite** | Option<**String**> | The path that replaces `path` before the request reaches the service. Null when the path is not rewritten. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


