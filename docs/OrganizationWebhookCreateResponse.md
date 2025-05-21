# OrganizationWebhookCreateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**kind** | Option<[**models::OrganizationWebhookKindEnum**](OrganizationWebhookKindEnum.md)> |  | [optional]
**target_url** | Option<**String**> | Set the public HTTP or HTTPS endpoint that will receive the specified events. The target URL must starts with `http://` or `https://`  | [optional]
**target_secret_set** | Option<**bool**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> | Turn on or off your endpoint. | [optional]
**events** | Option<[**Vec<models::OrganizationWebhookEventEnum>**](OrganizationWebhookEventEnum.md)> |  | [optional]
**project_names_filter** | Option<**Vec<String>**> | Specify the project names you want to filter to.  This webhook will be triggered only if the event is coming from the specified Project IDs. Notes: 1. Wildcard is accepted E.g. `product*`. 2. Name is case insensitive.  | [optional]
**environment_types_filter** | Option<[**Vec<models::EnvironmentModeEnum>**](EnvironmentModeEnum.md)> | Specify the environment modes you want to filter to. This webhook will be triggered only if the event is coming from an environment with the specified mode.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


