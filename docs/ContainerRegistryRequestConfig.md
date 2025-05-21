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
**json_credentials** | Option<**String**> | Required if kind is `GCP_ARTIFACT_REGISTRY` | [optional]
**username** | Option<**String**> | optional, for kind `DOCKER_HUB`   We encourage you to set credentials for Docker Hub due to the limits on the pull rate  | [optional]
**password** | Option<**String**> | optional, for kind `DOCKER_HUB`   We encourage you to set credentials for Docker Hub due to the limits on the pull rate  | [optional]
**role_arn** | Option<**String**> | For ECR, you can either set a static access_key or use a role arn that we are going to assume | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


