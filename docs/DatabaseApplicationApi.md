# \DatabaseApplicationApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_database_application**](DatabaseApplicationApi.md#list_database_application) | **GET** /database/{databaseId}/application | List applications using the database
[**remove_application_from_database**](DatabaseApplicationApi.md#remove_application_from_database) | **DELETE** /database/{databaseId}/application/{targetApplicationId} | Remove an application from this database 



## list_database_application

> models::ApplicationResponseList list_database_application(database_id)
List applications using the database

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **uuid::Uuid** | Database ID | [required] |

### Return type

[**models::ApplicationResponseList**](ApplicationResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_application_from_database

> remove_application_from_database(database_id, target_application_id)
Remove an application from this database 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **uuid::Uuid** | Database ID | [required] |
**target_application_id** | **uuid::Uuid** | Target application ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

