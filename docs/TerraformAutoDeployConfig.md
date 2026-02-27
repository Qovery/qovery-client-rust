# TerraformAutoDeployConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auto_deploy** | **bool** |  | 
**auto_deploy_action** | **String** | Action to force a specific Terraform behavior on autodeploy. `DEFAULT`: The action is resolved based on the deployment type:   - Start/Restart -> PLAN_AND_APPLY   - Delete -> DESTROY   - Pause -> PLAN_ONLY  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


