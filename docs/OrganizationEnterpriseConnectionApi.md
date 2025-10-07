# \OrganizationEnterpriseConnectionApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_organization_enterprise_connection**](OrganizationEnterpriseConnectionApi.md#get_organization_enterprise_connection) | **GET** /organization/{organizationId}/enterpriseconnection/{connectionName} | Get enterprise connection
[**list_organization_enterprise_connections**](OrganizationEnterpriseConnectionApi.md#list_organization_enterprise_connections) | **GET** /organization/{organizationId}/enterpriseconnection | List enterprise connections
[**update_organization_enterprise_connection**](OrganizationEnterpriseConnectionApi.md#update_organization_enterprise_connection) | **PUT** /organization/{organizationId}/enterpriseconnection/{connectionName} | Update enterprise connection



## get_organization_enterprise_connection

> models::EnterpriseConnectionDto get_organization_enterprise_connection(organization_id, connection_name)
Get enterprise connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**connection_name** | **String** | The name of the Organization's Enterprise Connection | [required] |

### Return type

[**models::EnterpriseConnectionDto**](EnterpriseConnectionDto.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_enterprise_connections

> models::EnterpriseConnectionResponseList list_organization_enterprise_connections(organization_id)
List enterprise connections

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::EnterpriseConnectionResponseList**](EnterpriseConnectionResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization_enterprise_connection

> models::EnterpriseConnectionDto update_organization_enterprise_connection(organization_id, connection_name, enterprise_connection_dto)
Update enterprise connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**connection_name** | **String** | The name of the Organization's Enterprise Connection | [required] |
**enterprise_connection_dto** | Option<[**EnterpriseConnectionDto**](EnterpriseConnectionDto.md)> |  |  |

### Return type

[**models::EnterpriseConnectionDto**](EnterpriseConnectionDto.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

