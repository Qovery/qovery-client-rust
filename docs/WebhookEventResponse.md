# WebhookEventResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier | 
**created_at** | **String** | Timestamp when the webhook event was created | 
**kind** | [**models::OrganizationWebhookKindEnum**](OrganizationWebhookKindEnum.md) |  | 
**matched_event** | [**models::OrganizationWebhookEventEnum**](OrganizationWebhookEventEnum.md) |  | 
**target_url_used** | **String** | The webhook target URL that was invoked | 
**request** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | The request payload sent to the webhook | 
**target_response_status_code** | **i32** | HTTP status code returned by the webhook target | 
**target_response_body** | Option<**String**> | Response body from the webhook target | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


