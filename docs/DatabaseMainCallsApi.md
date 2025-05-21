# \DatabaseMainCallsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_database**](DatabaseMainCallsApi.md#delete_database) | **DELETE** /database/{databaseId} | Delete a database 
[**edit_database**](DatabaseMainCallsApi.md#edit_database) | **PUT** /database/{databaseId} | Edit a database 
[**edit_database_credentials**](DatabaseMainCallsApi.md#edit_database_credentials) | **PUT** /database/{databaseId}/masterCredentials | Edit database  master credentials
[**get_database**](DatabaseMainCallsApi.md#get_database) | **GET** /database/{databaseId} | Get database by ID
[**get_database_master_credentials**](DatabaseMainCallsApi.md#get_database_master_credentials) | **GET** /database/{databaseId}/masterCredentials | Get master credentials of the database
[**get_database_status**](DatabaseMainCallsApi.md#get_database_status) | **GET** /database/{databaseId}/status | Get database status
[**list_database_version**](DatabaseMainCallsApi.md#list_database_version) | **GET** /database/{databaseId}/version | List eligible versions for the database



## delete_database

> delete_database(database_id)
Delete a database 

To delete a database you must have the admin permission

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **uuid::Uuid** | Database ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_database

> models::Database edit_database(database_id, database_edit_request)
Edit a database 

To edit a database  you must have the admin permission

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **uuid::Uuid** | Database ID | [required] |
**database_edit_request** | Option<[**DatabaseEditRequest**](DatabaseEditRequest.md)> |  |  |

### Return type

[**models::Database**](Database.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_database_credentials

> models::Credentials edit_database_credentials(database_id, credentials_request)
Edit database  master credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **uuid::Uuid** | Database ID | [required] |
**credentials_request** | Option<[**CredentialsRequest**](CredentialsRequest.md)> |  |  |

### Return type

[**models::Credentials**](Credentials.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_database

> models::Database get_database(database_id)
Get database by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **uuid::Uuid** | Database ID | [required] |

### Return type

[**models::Database**](Database.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_database_master_credentials

> models::Credentials get_database_master_credentials(database_id)
Get master credentials of the database

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **uuid::Uuid** | Database ID | [required] |

### Return type

[**models::Credentials**](Credentials.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_database_status

> models::Status get_database_status(database_id)
Get database status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **uuid::Uuid** | Database ID | [required] |

### Return type

[**models::Status**](Status.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_database_version

> models::VersionResponseList list_database_version(database_id)
List eligible versions for the database

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **uuid::Uuid** | Database ID | [required] |

### Return type

[**models::VersionResponseList**](VersionResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

