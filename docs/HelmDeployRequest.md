# HelmDeployRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chart_version** | Option<**String**> | version of the chart to deploy. Cannot be set if `git_commit_id` is defined  | [optional]
**git_commit_id** | Option<**String**> | Commit to deploy for chart source. Cannot be set if `version` is defined  | [optional]
**values_override_git_commit_id** | Option<**String**> | Commit to deploy for values override  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


