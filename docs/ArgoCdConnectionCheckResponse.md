# ArgoCdConnectionCheckResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **Status** | Connection result (enum: connected, error) | 
**app_count** | Option<**i32**> | Number of ArgoCD applications visible with the provided token. Present only when status is \"connected\". | [optional]
**reason** | Option<**Reason**> | Failure reason. Present only when status is \"error\". (enum: authentication_failed, unreachable, insufficient_permissions) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


