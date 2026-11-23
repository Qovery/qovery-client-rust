# ServiceDomainCustomResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [**models::ServiceDomainKindEnum**](ServiceDomainKindEnum.md) |  | 
**domain** | **String** |  | 
**generate_certificate** | **bool** | Whether Qovery generates a certificate for the domain. | 
**use_cdn** | **bool** | Whether the domain is behind a CDN. | 
**cname_targets** | **Vec<String>** | The hostnames the domain must point to with a CNAME record. One value, except for a legacy domain spread over several service routers. | 
**generated_hostnames** | [**Vec<models::GeneratedHostnameResponse>**](GeneratedHostnameResponse.md) | The hostnames served by the routes of this service only. | 
**routes** | [**Vec<models::CustomDomainRouteResponse>**](CustomDomainRouteResponse.md) | Every route of the domain, the routes of the other services included. | 
**other_services** | [**Vec<models::CustomDomainServiceResponse>**](CustomDomainServiceResponse.md) | The other services routing the domain. When it is not empty, removing this service's route keeps the domain for them. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


