# TerraformResourceResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier for this resource record | 
**resource_type** | **String** | Type of the Terraform resource (e.g., aws_instance, aws_s3_bucket) | 
**name** | **String** | Name of the resource as defined in Terraform configuration | 
**address** | **String** | Full address of the resource (e.g., aws_instance.web_server) | 
**provider** | **String** | Terraform provider name (e.g., aws, google, azurerm) | 
**mode** | **String** | Resource mode (managed or data source) | 
**attributes** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | All resource attributes as key-value pairs | 
**extracted_at** | **String** | Timestamp when the resource was extracted from Terraform state | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


