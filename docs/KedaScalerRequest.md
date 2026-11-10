# KedaScalerRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scaler_type** | **String** |  | 
**enabled** | Option<**bool**> |  | [optional]
**role** | [**models::KedaScalerRole**](KedaScalerRole.md) |  | 
**config_json** | Option<**serde_json::Value**> | Scaler `metadata` only, as an object of string values, e.g. `{\"query\": \"...\", \"threshold\": \"10\"}`. Other trigger fields (`name`, `metricType`, `useCachedMetrics`) cannot be set here: use `config_yaml`. Exactly one of `config_json` and `config_yaml` must be set. | [optional]
**config_yaml** | Option<**String**> | Body of the KEDA trigger as YAML, without `type` (set from `scaler_type`) and `authenticationRef` (set from `trigger_authentication`). Scaler settings go under `metadata:`; `name`, `metricType` and `useCachedMetrics` can be set next to it. Exactly one of `config_json` and `config_yaml` must be set. | [optional]
**trigger_authentication** | Option<[**models::KedaTriggerAuthenticationRequest**](KedaTriggerAuthenticationRequest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


