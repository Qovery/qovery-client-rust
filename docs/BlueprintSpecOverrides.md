# BlueprintSpecOverrides

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**engine_version** | Option<**String**> | Terraform or OpenTofu version to use for the apply job. Required when the blueprint engine type is `terraform` or `opentofu`. Must be one of the versions in the manifest's `allowedValues` list. | [optional]
**credentials** | Option<**Credentials**> | How the apply job authenticates against the cloud provider. `cluster` reuses the cluster's cloud credentials (default). `env` expects the user to supply provider credentials as environment variables on the service. (enum: cluster, env) | [optional]
**backend** | Option<**Backend**> | Where the Terraform state is stored. `qovery` stores state in a Kubernetes secret managed by Qovery (default). `user_provided` delegates to a user-controlled remote backend declared in the manifest's `backend.user_provided` block. (enum: qovery, user_provided) | [optional]
**timeout** | Option<**i32**> | Maximum duration in seconds for a single apply job before it is marked as timed out. Overrides the manifest's `spec.engine.timeout`. | [optional]
**cpu** | Option<**String**> | CPU request/limit for the apply job pod (Kubernetes-style, e.g. `500m`). Overrides `spec.engine.resources.cpu` in the manifest. | [optional]
**ram** | Option<**String**> | Memory request/limit for the apply job pod (e.g. `512Mi`, `1Gi`). Overrides `spec.engine.resources.ram` in the manifest. | [optional]
**storage** | Option<**String**> | Ephemeral storage for the apply job pod — used for state files and provider plugins (e.g. `1Gi`). Overrides `spec.engine.resources.storage` in the manifest. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


