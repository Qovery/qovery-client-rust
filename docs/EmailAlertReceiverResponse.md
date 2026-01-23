# EmailAlertReceiverResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | **String** |  | 
**description** | **String** |  | 
**r#type** | [**models::AlertReceiverType**](AlertReceiverType.md) |  | 
**send_resolved** | **bool** |  | 
**to** | **String** |  | 
**from** | **String** |  | 
**smarthost** | **String** |  | 
**auth_username** | Option<**String**> |  | 
**require_tls** | **bool** |  | 
**owner** | Option<**String**> |  | [optional]
**severity** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


