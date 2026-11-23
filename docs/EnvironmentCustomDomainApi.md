# \EnvironmentCustomDomainApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_environment_custom_domain**](EnvironmentCustomDomainApi.md#create_environment_custom_domain) | **POST** /environment/{environmentId}/customDomain | Create a custom domain for several services
[**delete_environment_custom_domain**](EnvironmentCustomDomainApi.md#delete_environment_custom_domain) | **DELETE** /environment/{environmentId}/customDomain/{domainName} | Delete a custom domain of several services
[**edit_environment_custom_domain**](EnvironmentCustomDomainApi.md#edit_environment_custom_domain) | **PUT** /environment/{environmentId}/customDomain/{domainName} | Edit a custom domain of several services
[**get_environment_custom_domain_cname_target**](EnvironmentCustomDomainApi.md#get_environment_custom_domain_cname_target) | **GET** /environment/{environmentId}/customDomain/cnameTarget | Get the CNAME target of the environment
[**list_environment_custom_domain**](EnvironmentCustomDomainApi.md#list_environment_custom_domain) | **GET** /environment/{environmentId}/customDomain | List environment custom domains
[**validate_environment_custom_domain**](EnvironmentCustomDomainApi.md#validate_environment_custom_domain) | **POST** /environment/{environmentId}/customDomain/validate | Validate a custom domain for several services



## create_environment_custom_domain

> models::EnvironmentCustomDomainResponse create_environment_custom_domain(environment_id, environment_custom_domain_request)
Create a custom domain for several services

Create a custom domain on several services of the environment at once: one custom domain and one `/` route per service, served by the service default port. If any service conflicts, nothing is created and the response lists every conflict. More than one service is refused with a 400 on an environment whose `routing_capabilities.can_share` is `false`. Nothing is deployed: redeploy the services listed in `impacted_services` to serve the domain. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**environment_custom_domain_request** | [**EnvironmentCustomDomainRequest**](EnvironmentCustomDomainRequest.md) |  | [required] |

### Return type

[**models::EnvironmentCustomDomainResponse**](EnvironmentCustomDomainResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_environment_custom_domain

> models::CustomDomainDeletionResponse delete_environment_custom_domain(environment_id, domain_name)
Delete a custom domain of several services

Delete every custom domain row of `domainName` in the environment, for every service routing it, with their routes. To remove the domain from one service only, use the delete endpoint of that service. Nothing is deployed: redeploy the services listed in `impacted_services` to apply the change. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**domain_name** | **String** | The current name of the custom domain. | [required] |

### Return type

[**models::CustomDomainDeletionResponse**](CustomDomainDeletionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_environment_custom_domain

> models::EnvironmentCustomDomainResponse edit_environment_custom_domain(environment_id, domain_name, environment_custom_domain_request)
Edit a custom domain of several services

Replace every custom domain row of `domainName` in the environment with the state of the body: * services not in the body lose their row and its routes; * services new in the body get a row and a `/` route; * kept services get the name, `generate_certificate` and `use_cdn` of the body. Renaming is allowed and is checked like a creation. If any service conflicts, nothing changes and the response lists every conflict. Nothing is deployed: redeploy the services listed in `impacted_services` to apply the change. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**domain_name** | **String** | The current name of the custom domain. | [required] |
**environment_custom_domain_request** | [**EnvironmentCustomDomainRequest**](EnvironmentCustomDomainRequest.md) |  | [required] |

### Return type

[**models::EnvironmentCustomDomainResponse**](EnvironmentCustomDomainResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_environment_custom_domain_cname_target

> models::EnvironmentCnameTargetResponse get_environment_custom_domain_cname_target(environment_id)
Get the CNAME target of the environment

The hostname a new custom domain of the environment must point to with a CNAME record. Returns 404 until the environment is deployed for the first time and has its router.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |

### Return type

[**models::EnvironmentCnameTargetResponse**](EnvironmentCnameTargetResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_environment_custom_domain

> models::OrganizationCustomDomainResponseList list_environment_custom_domain(environment_id)
List environment custom domains

List the custom domains of the environment. Each result is one domain name, with every route that serves it: a domain shared by several services is a single result. The items are the same as in the organization list. The list is not paginated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |

### Return type

[**models::OrganizationCustomDomainResponseList**](OrganizationCustomDomainResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_environment_custom_domain

> models::CustomDomainErrorsResponse validate_environment_custom_domain(environment_id, custom_domain_validation_request)
Validate a custom domain for several services

Run the checks of a custom domain creation or edit for several services, without saving anything. Returns every conflict, not only the first one. The services of the body are checked against each other as well as against the existing domains. It takes no lock: a later creation can still fail if the domains change meanwhile. A service outside the environment returns a 403. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment_id** | **uuid::Uuid** | Environment ID | [required] |
**custom_domain_validation_request** | [**CustomDomainValidationRequest**](CustomDomainValidationRequest.md) |  | [required] |

### Return type

[**models::CustomDomainErrorsResponse**](CustomDomainErrorsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

