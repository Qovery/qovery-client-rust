# DeployAllRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**applications** | Option<[**Vec<models::DeployAllRequestApplicationsInner>**](DeployAllRequestApplicationsInner.md)> |  | [optional]
**databases** | Option<**Vec<uuid::Uuid>**> |  | [optional]
**containers** | Option<[**Vec<models::DeployAllRequestContainersInner>**](DeployAllRequestContainersInner.md)> |  | [optional]
**jobs** | Option<[**Vec<models::DeployAllRequestJobsInner>**](DeployAllRequestJobsInner.md)> |  | [optional]
**helms** | Option<[**Vec<models::DeployAllRequestHelmsInner>**](DeployAllRequestHelmsInner.md)> |  | [optional]
**terraforms** | Option<[**Vec<models::TerraformDeployRequest>**](TerraformDeployRequest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


