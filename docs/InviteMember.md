# InviteMember

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**email** | **String** |  | 
**role** | [**models::InviteMemberRoleEnum**](InviteMemberRoleEnum.md) |  | 
**invitation_link** | **String** |  | 
**invitation_status** | [**models::InviteStatusEnum**](InviteStatusEnum.md) |  | 
**organization_name** | Option<**String**> |  | [optional]
**inviter** | **String** |  | 
**logo_url** | Option<**String**> |  | [optional]
**role_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**role_name** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


