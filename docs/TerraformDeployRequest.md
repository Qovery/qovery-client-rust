# TerraformDeployRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Terraform service identifier | [optional]
**git_commit_id** | Option<**String**> | Commit to deploy for chart source. | [optional]
**dry_run** | Option<**bool**> | Deprecated: use action=PLAN instead. | [optional]
**force_unlock_state** | Option<**bool**> | Deprecated: use action=FORCE_UNLOCK instead. | [optional]
**action** | Option<**String**> | Terraform action to execute. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


