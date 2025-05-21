# CustomDomainRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain** | **String** | your custom domain | 
**generate_certificate** | **bool** | to control if a certificate has to be generated for this custom domain by Qovery. The default value is `true`. This flag should be set to `false` if a CDN or other entities are managing the certificate for the specified domain and the traffic is proxied by the CDN to Qovery. | 
**use_cdn** | Option<**bool**> | Indicates if the custom domain is behind a CDN (i.e Cloudflare). This will condition the way we are checking CNAME before & during a deployment: * If `true` then we only check the domain points to an IP * If `false` then we check that the domain resolves to the correct service Load Balancer  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


