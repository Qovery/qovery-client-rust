# ClusterLogs

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | log level | [optional]
**timestamp** | Option<**String**> | log date creation | [optional]
**step** | Option<**Step**> | log step (enum: LoadConfiguration, Create, Created, CreateError, Pause, Paused, PauseError, Delete, Deleted, DeleteError, RetrieveClusterConfig, RetrieveClusterResources, ValidateSystemRequirements, UnderMigration, Unknown) | [optional]
**message** | Option<[**models::ClusterLogsMessage**](ClusterLogsMessage.md)> |  | [optional]
**error** | Option<[**models::ClusterLogsError**](ClusterLogsError.md)> |  | [optional]
**details** | Option<[**models::ClusterLogsDetails**](ClusterLogsDetails.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


