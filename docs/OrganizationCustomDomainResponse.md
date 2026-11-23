# OrganizationCustomDomainResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Id of the environment of the domain. | 
**domain** | **String** | The domain name. | 
**project** | [**models::CustomDomainProjectResponse**](CustomDomainProjectResponse.md) |  | 
**environment** | [**models::CustomDomainEnvironmentResponse**](CustomDomainEnvironmentResponse.md) |  | 
**routes** | [**Vec<models::CustomDomainRouteResponse>**](CustomDomainRouteResponse.md) | The routes of the domain, one per service and path. | 
**generate_certificate** | **bool** | Whether Qovery generates a certificate for the domain. `false` when a CDN or another party manages the certificate. | 
**use_cdn** | **bool** | Whether the domain is behind a CDN. If `true`, Qovery only checks that the domain points to an IP; otherwise it checks that the domain resolves to the service load balancer. | 
**cname_targets** | Option<**Vec<String>**> | The hostnames the domain must point to with a CNAME record. One value, except for a legacy domain spread over several service routers. | [optional]
**generated_hostnames** | Option<[**Vec<models::GeneratedHostnameResponse>**](GeneratedHostnameResponse.md)> | Every hostname served for the domain by its routes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


