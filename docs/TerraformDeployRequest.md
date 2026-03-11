# TerraformDeployRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**uuid::Uuid**> | Terraform service identifier | [optional]
**git_commit_id** | Option<**String**> | Commit to deploy for chart source. | [optional]
**action** | Option<**Action**> | Terraform action to execute. (enum: PLAN, FORCE_UNLOCK, MIGRATE_STATE) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


