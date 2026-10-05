# \OrganizationEnterpriseConnectionApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_enterprise_connection_roles**](OrganizationEnterpriseConnectionApi.md#get_enterprise_connection_roles) | **GET** /account/enterpriseconnection/roles | Resolve enterprise connection roles
[**get_organization_enterprise_connection**](OrganizationEnterpriseConnectionApi.md#get_organization_enterprise_connection) | **GET** /organization/{organizationId}/enterpriseconnection/{connectionName} | Get enterprise connection
[**list_organization_enterprise_connections**](OrganizationEnterpriseConnectionApi.md#list_organization_enterprise_connections) | **GET** /organization/{organizationId}/enterpriseconnection | List enterprise connections
[**notify_enterprise_member_access_updated**](OrganizationEnterpriseConnectionApi.md#notify_enterprise_member_access_updated) | **POST** /account/enterpriseconnection/notifyMemberAccessUpdated | Notify enterprise member access changes
[**update_organization_enterprise_connection**](OrganizationEnterpriseConnectionApi.md#update_organization_enterprise_connection) | **PUT** /organization/{organizationId}/enterpriseconnection/{connectionName} | Update enterprise connection



## get_enterprise_connection_roles

> models::EnterpriseConnectionAccessList get_enterprise_connection_roles(x_qovery_auth0_post_login_token, connection_name, federated_groups, user_sub)
Resolve enterprise connection roles

Resolve organization access for an Auth0 post-login action.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_qovery_auth0_post_login_token** | **String** |  | [required] |
**connection_name** | **String** |  | [required] |
**federated_groups** | **String** |  | [required] |
**user_sub** | **String** |  | [required] |

### Return type

[**models::EnterpriseConnectionAccessList**](EnterpriseConnectionAccessList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## notify_enterprise_member_access_updated

> notify_enterprise_member_access_updated(x_qovery_auth0_post_login_token, enterprise_connection_member_access_update_request)
Notify enterprise member access changes

Notify q-core of member access changes from an Auth0 post-login action.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_qovery_auth0_post_login_token** | **String** |  | [required] |
**enterprise_connection_member_access_update_request** | [**EnterpriseConnectionMemberAccessUpdateRequest**](EnterpriseConnectionMemberAccessUpdateRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

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

