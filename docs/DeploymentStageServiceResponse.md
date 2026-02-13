# DeploymentStageServiceResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**service_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | id of the service attached to the stage | [optional]
**service_type** | Option<**String**> | type of the service (i.e APPLICATION, JOB, DATABASE, ...) | [optional]
**is_skipped** | Option<**bool**> | whether the service is excluded from environment-level deployments | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


