# BlueprintDetailsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | 
**name** | **String** |  | 
**catalog_url** | **String** | URL to the blueprint catalog entry | 
**tag** | **String** |  | 
**environment_id** | **uuid::Uuid** |  | 
**service_type** | **ServiceType** | Type of the underlying service backing this blueprint (enum: HELM, TERRAFORM) | 
**service_id** | Option<**uuid::Uuid**> | The service the dispatch produced. Null while the dispatch is still running, and null if it failed. | 
**latest_deployment** | Option<[**models::BlueprintDeploymentStatusResponse**](BlueprintDeploymentStatusResponse.md)> | Latest dispatch for this blueprint. Null if it was never dispatched. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


