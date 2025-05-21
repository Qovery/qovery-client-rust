# \BackupsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_backup_database**](BackupsApi.md#add_backup_database) | **POST** /database/{databaseId}/backup | Add a backup to the Database 
[**list_database_backup**](BackupsApi.md#list_database_backup) | **GET** /database/{databaseId}/backup | List database  backups
[**remove_database_backup**](BackupsApi.md#remove_database_backup) | **DELETE** /database/{databaseId}/backup/{backupId} | Remove database  backup



## add_backup_database

> models::Backup add_backup_database(database_id, backup_request)
Add a backup to the Database 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **uuid::Uuid** | Database ID | [required] |
**backup_request** | Option<[**BackupRequest**](BackupRequest.md)> |  |  |

### Return type

[**models::Backup**](Backup.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_database_backup

> models::BackupPaginatedResponseList list_database_backup(database_id, start_id)
List database  backups

By default it returns the 20 last results. The response is paginated. In order to request the next page, you can use the startId query parameter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **uuid::Uuid** | Database ID | [required] |
**start_id** | Option<**uuid::Uuid**> | Starting point after which to return results |  |

### Return type

[**models::BackupPaginatedResponseList**](BackupPaginatedResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_database_backup

> remove_database_backup(database_id, backup_id)
Remove database  backup

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_id** | **uuid::Uuid** | Database ID | [required] |
**backup_id** | **uuid::Uuid** | Database Backup ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

