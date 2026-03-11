# LifecycleTemplateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | 
**name** | **String** |  | 
**description** | **String** |  | 
**source_url** | **String** | location of the template | 
**cloud_provider** | [**models::CloudProviderEnum**](CloudProviderEnum.md) |  | 
**events** | [**Vec<models::LifecycleTemplateResponseEventsInner>**](LifecycleTemplateResponseEventsInner.md) | lis of pre-defined command for each event | 
**max_duration_in_sec** | **i32** | Job max allowed duration in seconds. After this allowed time, the job is going to be killed. | 
**resources** | [**models::LifecycleTemplateResponseResources**](LifecycleTemplateResponseResources.md) |  | 
**variables** | [**Vec<models::LifecycleTemplateResponseVariablesInner>**](LifecycleTemplateResponseVariablesInner.md) | Variables to inject at the creation of this lifecycle job | 
**dockerfile** | **String** | Dockerfile of the template | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


