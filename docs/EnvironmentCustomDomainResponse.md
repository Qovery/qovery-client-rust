# EnvironmentCustomDomainResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain** | **String** | The domain name. | 
**generate_certificate** | **bool** | Whether Qovery generates a certificate for the domain. | 
**use_cdn** | **bool** | Whether the domain is behind a CDN. | 
**impacted_services** | [**Vec<models::ImpactedServiceResponse>**](ImpactedServiceResponse.md) | The services whose custom domains changed, removed services included. They need a redeploy to serve the change. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


