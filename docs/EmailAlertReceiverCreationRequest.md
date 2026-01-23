# EmailAlertReceiverCreationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**name** | **String** |  | 
**description** | **String** |  | 
**r#type** | [**models::AlertReceiverType**](AlertReceiverType.md) |  | 
**send_resolved** | **bool** |  | 
**owner** | Option<**String**> |  | [optional]
**severity** | Option<**String**> |  | [optional]
**to** | **String** | Recipient email address | 
**from** | **String** | Sender email address | 
**smarthost** | **String** | SMTP server in format 'host:port' | 
**auth_username** | Option<**String**> | SMTP authentication username. Defaults to 'from' if not provided. | [optional]
**auth_password** | **String** | SMTP authentication password | 
**require_tls** | **bool** | Whether to require TLS for SMTP connection | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


