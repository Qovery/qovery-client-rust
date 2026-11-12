# KedaAutoscalingResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**service_id** | **uuid::Uuid** |  | 
**mode** | [**models::AutoscalingMode**](AutoscalingMode.md) |  | 
**polling_interval_seconds** | **i32** |  | 
**cooldown_period_seconds** | **i32** |  | 
**initial_cooldown_period_seconds** | Option<**i32**> | KEDA ScaledObject `initialCooldownPeriod`: seconds to wait after the ScaledObject is created before the service can scale to zero. Omitted or null leaves KEDA's default (0). | [optional]
**idle_replica_count** | Option<**i32**> | KEDA ScaledObject `idleReplicaCount`: replicas to keep while no trigger is active. Only 0 is supported by KEDA, and it must be strictly less than the service's minimum running instances. Omitted or null disables the idle state. | [optional]
**advanced_yaml** | Option<**String**> | Raw YAML of the KEDA ScaledObject `advanced` field, e.g. `horizontalPodAutoscalerConfig.behavior` to tune how fast the HPA scales up and down. Must be a YAML mapping whose top-level keys are among `horizontalPodAutoscalerConfig`, `restoreToOriginalReplicaCount` and `scalingModifiers`. Omitted, null or blank means no advanced settings. | [optional]
**scalers** | [**Vec<models::KedaScalerResponse>**](KedaScalerResponse.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


