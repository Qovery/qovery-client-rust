# GetClusterKubernetesEvents200ResponseResultsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | The created date following ISO-8601 format | [optional]
**kind** | Option<**String**> | The source kubernetes object related to the Event | [optional]
**namespace** | Option<**String**> | The namespace of the kubernetes object related to the Event (optional) | [optional]
**name** | Option<**String**> | The name of the Event | [optional]
**reason** | Option<**String**> | The action that triggered the Event | [optional]
**message** | Option<**String**> | A description of the Event | [optional]
**r#type** | Option<**String**> | As of today it can be either Warning or Normal (can evolve in the next releases) | [optional]
**reporting_component** | Option<**String**> |  | [optional]
**count** | Option<**i32**> |  | [optional]
**first_occurrence** | Option<**String**> |  | [optional]
**last_occurrence** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


