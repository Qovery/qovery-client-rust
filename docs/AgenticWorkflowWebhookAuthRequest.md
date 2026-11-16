# AgenticWorkflowWebhookAuthRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mode** | [**models::AgenticWorkflowWebhookAuthMode**](AgenticWorkflowWebhookAuthMode.md) |  | 
**secret** | Option<**String**> | Write-only, never returned. Ignored for `NONE`.  For `SECRET_HEADER`, the secret the caller will send. It must be 16 to 1024 printable ASCII characters, without leading or trailing spaces. Required when entering the mode. Once in the mode, omit it (or send a blank value) to keep the current secret; send a new one to replace it, which is how the secret is rotated. Only its SHA-256 hash is stored.  For the provider modes, the provider's webhook signing secret. Required when entering the mode; omitted or blank keeps the stored one while the mode does not change.  An invalid or missing secret is rejected with 400 on `webhook_auth.secret`.  | [optional]
**header_name** | Option<**String**> | `SECRET_HEADER` only: the HTTP header that carries the secret. Must be a valid header name (RFC 9110 token) of at most 128 characters, and not a header that proxies rewrite or consume (`authorization`, `cookie`, `host`, `content-type`, `x-forwarded-*`...). It is trimmed and stored lowercased. Omitted or blank keeps the current name, or uses `x-qovery-webhook-secret` when entering the mode. An invalid name is rejected with 400 on `webhook_auth.header_name`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


