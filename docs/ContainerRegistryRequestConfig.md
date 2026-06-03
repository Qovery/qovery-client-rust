# ContainerRegistryRequestConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_key_id** | Option<**String**> | Required if kind is `ECR` or `PUBLIC_ECR` | [optional]
**secret_access_key** | Option<**String**> | Required if kind is `ECR` or `PUBLIC_ECR` | [optional]
**region** | Option<**String**> | Required if kind is `ECR` or `SCALEWAY_CR` | [optional]
**scaleway_access_key** | Option<**String**> | Required if kind is `SCALEWAY_CR` | [optional]
**scaleway_secret_key** | Option<**String**> | Required if kind is `SCALEWAY_CR` | [optional]
**scaleway_project_id** | Option<**String**> | Required if kind is `SCALEWAY_CR` | [optional]
**json_credentials** | Option<**String**> | Required if kind is `GCP_ARTIFACT_REGISTRY`. For GCP Artifact Registry, you can either set a service account JSON key with json_credentials or use Workload Identity Federation with gcp_credentials_type. | [optional]
**gcp_credentials_type** | Option<**GcpCredentialsType**> | For GCP Artifact Registry, you can either set a service account JSON key with json_credentials or use Workload Identity Federation with gcp_credentials_type. (enum: workload_identity_federation) | [optional]
**project_id** | Option<**String**> | Required if kind is `GCP_ARTIFACT_REGISTRY` and gcp_credentials_type is `workload_identity_federation` | [optional]
**service_account_email** | Option<**String**> | Required if kind is `GCP_ARTIFACT_REGISTRY` and gcp_credentials_type is `workload_identity_federation` | [optional]
**workload_identity_project_number** | Option<**String**> | Required if kind is `GCP_ARTIFACT_REGISTRY` and gcp_credentials_type is `workload_identity_federation` | [optional]
**workload_identity_pool_id** | Option<**String**> | Required if kind is `GCP_ARTIFACT_REGISTRY` and gcp_credentials_type is `workload_identity_federation` | [optional]
**workload_identity_provider_id** | Option<**String**> | Required if kind is `GCP_ARTIFACT_REGISTRY` and gcp_credentials_type is `workload_identity_federation` | [optional]
**token_lifetime_seconds** | Option<**i32**> | Optional if kind is `GCP_ARTIFACT_REGISTRY` and gcp_credentials_type is `workload_identity_federation` | [optional]
**username** | Option<**String**> | optional, for kind `DOCKER_HUB`   We encourage you to set credentials for Docker Hub due to the limits on the pull rate  | [optional]
**password** | Option<**String**> | optional, for kind `DOCKER_HUB`   We encourage you to set credentials for Docker Hub due to the limits on the pull rate  | [optional]
**role_arn** | Option<**String**> | For ECR, you can either set a static access_key or use a role arn that we are going to assume | [optional]
**azure_tenant_id** | Option<**String**> | Required if kind is `AZURE_CR`. | [optional]
**azure_subscription_id** | Option<**String**> | Required if kind is `AZURE_CR`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


