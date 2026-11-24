# SelfManagedClusterRegistryRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | **Kind** | Kind of the registry. `GENERIC_CR` is any OCI registry reachable by URL, `GITHUB_CR` is GitHub Container Registry and `DOCKER_HUB` is Docker Hub; both require `credentials`. (enum: GENERIC_CR, GITHUB_CR, DOCKER_HUB) | 
**url** | Option<**String**> | URL of the registry. Required for `GENERIC_CR`: `http` or `https` scheme, host and optional port only, without user info, path, query or fragment, and not localhost. Optional for `GITHUB_CR` and `DOCKER_HUB`; when given, it must be `https://ghcr.io` or `https://docker.io`. | [optional]
**credentials** | Option<[**models::SelfManagedClusterRegistryCredentialsRequest**](SelfManagedClusterRegistryCredentialsRequest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


