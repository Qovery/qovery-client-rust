# OrganizationCustomDomainServiceCheck

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**service_id** | **uuid::Uuid** | Id of the service. | 
**service_type** | [**models::ServiceTypeEnum**](ServiceTypeEnum.md) | Type of the service. Only `APPLICATION`, `CONTAINER` and `HELM` services have custom domains. | 
**status** | Option<[**models::CheckedCustomDomainStatus**](CheckedCustomDomainStatus.md)> | The result of the check. `null` when the check could not run (cluster unreachable, timeout, service never deployed); `error_details` then gives the reason. | 
**error_details** | Option<**String**> | Why the check failed or could not run. `null` when the check succeeded. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


