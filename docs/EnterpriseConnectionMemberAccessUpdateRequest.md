# EnterpriseConnectionMemberAccessUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | **String** |  | 
**user_email** | Option<**String**> |  | [optional]
**added_organization_ids** | **Vec<uuid::Uuid>** |  | 
**removed_organization_ids** | **Vec<uuid::Uuid>** |  | 
**role_updates_by_organization_id** | [**Vec<models::MemberAccessRoleUpdated>**](MemberAccessRoleUpdated.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


