# ArgoCdConnectionCheckResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **String** | Connection result | 
**app_count** | Option<**i32**> | Number of ArgoCD applications visible with the provided token. Present only when status is \"connected\". | [optional]
**reason** | Option<**String**> | Failure reason. Present only when status is \"error\". | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


