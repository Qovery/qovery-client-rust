# EmailAlertReceiverEditRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**description** | **String** |  | 
**r#type** | [**models::AlertReceiverType**](AlertReceiverType.md) |  | 
**send_resolved** | **bool** |  | 
**owner** | Option<**String**> |  | [optional]
**severity** | Option<**String**> |  | [optional]
**to** | **String** |  | 
**from** | **String** |  | 
**smarthost** | **String** | SMTP server in format 'host:port' | 
**auth_username** | Option<**String**> |  | [optional]
**auth_password** | Option<**String**> | SMTP password. If null, keeps existing password. | [optional]
**require_tls** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


