# Member

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**nickname** | Option<**String**> |  | [optional]
**email** | **String** |  | 
**profile_picture_url** | Option<**String**> |  | [optional]
**last_activity_at** | Option<**String**> | last time the user was connected | [optional]
**role** | Option<[**models::InviteMemberRoleEnum**](InviteMemberRoleEnum.md)> |  | [optional]
**role_name** | Option<**String**> | the role linked to the user | [optional]
**role_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


