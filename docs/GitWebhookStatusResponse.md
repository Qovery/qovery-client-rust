# GitWebhookStatusResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **String** | The webhook configuration status: - ACTIVE: Webhook is properly configured with all required events - NOT_CONFIGURED: No Qovery webhook found on the git rep - MISCONFIGURED: Webhook exists but is missing required events - UNABLE_TO_VERIFY: Could not check webhook status (auth error, rate limit, etc.)  | 
**provider** | **String** | The git provider where the webhook is configured | 
**missing_events** | Option<**Vec<String>**> | List of required events that are missing from the webhook configuration (only present when status is MISCONFIGURED) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


