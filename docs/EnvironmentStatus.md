# EnvironmentStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**state** | [**models::StateEnum**](StateEnum.md) |  | 
**last_deployment_date** | Option<**String**> |  | [optional]
**last_deployment_state** | [**models::StateEnum**](StateEnum.md) |  | 
**last_deployment_id** | Option<**String**> |  | [optional]
**total_deployment_duration_in_seconds** | Option<**i32**> |  | [optional]
**origin** | Option<[**models::EnvironmentStatusEventOriginEnum**](EnvironmentStatusEventOriginEnum.md)> |  | [optional]
**triggered_by** | Option<**String**> |  | [optional]
**deployment_status** | Option<[**models::EnvironmentDeploymentStatusEnum**](EnvironmentDeploymentStatusEnum.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


