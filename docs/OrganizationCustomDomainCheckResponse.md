# OrganizationCustomDomainCheckResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Id of the environment of the domain. | 
**domain** | **String** | The domain name as `listOrganizationCustomDomain` returns it. A wildcard domain keeps its `*.` prefix. | 
**checks** | [**Vec<models::OrganizationCustomDomainServiceCheck>**](OrganizationCustomDomainServiceCheck.md) | One check per service the domain routes to. Empty when no service of the domain can be checked. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


