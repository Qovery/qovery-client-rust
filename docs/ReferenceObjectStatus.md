# ReferenceObjectStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**state** | [**models::StateEnum**](StateEnum.md) |  | 
**service_deployment_status** | [**models::ServiceDeploymentStatusEnum**](ServiceDeploymentStatusEnum.md) |  | 
**last_deployment_date** | Option<**String**> |  | [optional]
**is_part_last_deployment** | **bool** |  | 
**steps** | Option<[**models::ServiceStepMetrics**](ServiceStepMetrics.md)> |  | [optional]
**execution_id** | Option<**String**> |  | [optional]
**status_details** | [**models::StatusDetails**](StatusDetails.md) |  | 
**deployment_request_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | 
**deployment_requests_count** | **i32** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


