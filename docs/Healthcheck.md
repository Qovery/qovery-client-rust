# Healthcheck

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**readiness_probe** | Option<[**models::Probe**](Probe.md)> |  | [optional]
**liveness_probe** | Option<[**models::Probe**](Probe.md)> |  | [optional]
**startup_probe** | Option<[**models::Probe**](Probe.md)> | Optional probe checking that the container has started. Kubernetes runs the readiness and liveness probes only once it succeeds, so a slow start does not trip the liveness probe. Omitted or null means no startup probe. Like the other probes, an edit that leaves it out removes it. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


