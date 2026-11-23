# CustomDomainValidationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain** | **String** | The domain name. | 
**generate_certificate** | Option<**bool**> | Whether Qovery generates a certificate for the domain. Set it to `false` when a CDN or another party manages the certificate. | [optional][default to true]
**use_cdn** | Option<**bool**> | Whether the domain is behind a CDN. If `true`, Qovery only checks that the domain points to an IP; otherwise it checks that the domain resolves to the service load balancer. | [optional][default to false]
**services** | [**Vec<models::CustomDomainServiceRequest>**](CustomDomainServiceRequest.md) | Every service the domain routes to, each with one `/` route served by its default port. Between 1 and 50 services; more than one only when the environment `routing_capabilities.can_share` is `true`. | 
**original_domain** | Option<**String**> | The current name of a domain being edited, so the rows the services hold for it are ignored. Null or absent for a new domain. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


