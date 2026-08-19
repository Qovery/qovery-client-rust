# OrganizationPolicyApiTokenCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**role_name** | Option<**String**> |  | [optional]
**role_id** | Option<**uuid::Uuid**> | the organization role this token acts as once its policy has allowed a request. Effective access is the intersection of this role and its policy. | [optional]
**token** | Option<**String**> | the generated token to send in 'Authorization' header prefixed by 'Token '. It is returned only here and cannot be retrieved afterwards. | [optional]
**creator_name** | Option<**String**> |  | [optional]
**user_sub** | Option<**String**> |  | [optional]
**expires_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


