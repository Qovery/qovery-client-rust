# ArgocdAppResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | **String** | name is case insensitive | 
**service_type** | [**models::ServiceTypeEnum**](ServiceTypeEnum.md) |  | 
**namespace** | **String** |  | 
**environment_id** | **uuid::Uuid** |  | 
**cluster_id** | **uuid::Uuid** |  | 
**last_synced_at** | Option<**String**> |  | [optional]
**manifest_revision** | Option<**String**> |  | [optional]
**source_repo_url** | Option<**String**> |  | [optional]
**source_target_revision** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


