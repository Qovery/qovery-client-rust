# Service

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | uuid of the associated service (application, database, job, gateway...) | 
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**r#type** | Option<[**models::ServiceTypeEnum**](ServiceTypeEnum.md)> |  | [optional]
**name** | Option<**String**> | name of the service | [optional]
**deployed_commit_id** | Option<**String**> | Git commit ID corresponding to the deployed version of the application | [optional]
**last_updated_by** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | uuid of the user that made the last update | [optional]
**consumed_resources_in_percent** | Option<**f64**> | global overview of resources consumption of the service | [optional]
**service_typology** | Option<**String**> | describes the typology of service (container, postgresl, redis...) | [optional]
**service_version** | Option<**String**> | for databases this field exposes the database version | [optional]
**to_update** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


