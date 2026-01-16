# TerraformResourcesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**terraform_id** | [**uuid::Uuid**](uuid::Uuid.md) | ID of the Terraform service | 
**execution_id** | **String** | Execution ID in format \"UUID-version\" or \"UUID-version-timestamp\" Example: 550e8400-e29b-41d4-a716-446655440000-1  | 
**resources_json** | **String** | JSON array of terraform resources extracted from terraform show output. Each resource contains: resource_type, name, provider, address, mode, attributes  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


