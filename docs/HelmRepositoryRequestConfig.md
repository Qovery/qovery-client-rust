# HelmRepositoryRequestConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**username** | Option<**String**> | Required if the repository is private | [optional]
**password** | Option<**String**> | Required if the repository is private | [optional]
**access_key_id** | Option<**String**> | Required if kind is `ECR` or `PUBLIC_ECR` | [optional]
**secret_access_key** | Option<**String**> | Required if kind is `ECR` or `PUBLIC_ECR` | [optional]
**region** | Option<**String**> | Required if kind is `ECR` or `SCALEWAY_CR` | [optional]
**scaleway_access_key** | Option<**String**> | Required if kind is `SCALEWAY_CR` | [optional]
**scaleway_secret_key** | Option<**String**> | Required if kind is `SCALEWAY_CR` | [optional]
**scaleway_project_id** | Option<**String**> | Required if kind is `SCALEWAY_CR` | [optional]
**azure_tenant_id** | Option<**String**> | Required if kind is `AZURE_CR`. | [optional]
**azure_subscription_id** | Option<**String**> | Required if kind is `AZURE_CR`. | [optional]
**role_arn** | Option<**String**> | For ECR, you can either set a static access_key or use a role arn that we are going to assume | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


