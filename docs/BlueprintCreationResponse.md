# BlueprintCreationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | 
**catalog_url** | **String** | URL to the blueprint catalog entry | 
**tag** | **String** |  | 
**environment_id** | **uuid::Uuid** |  | 
**deployment_id** | **uuid::Uuid** | Identifier of the dispatch started by this creation. Resolve its status with GET /blueprint/{blueprintId}, and match it against `latest_deployment.id` there to confirm that dispatch is this one rather than a later re-dispatch. | 
**execution_id** | **String** | Engine execution identifier for this dispatch | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


