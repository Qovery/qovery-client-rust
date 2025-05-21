# OrganizationCustomRoleProjectPermissionsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**project_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**project_name** | Option<**String**> |  | [optional]
**is_admin** | Option<**bool**> | If `is_admin` is `true`, the user is: - automatically `MANAGER` for each environment type - allowed to manage project deployment rules - able to delete the project    Note that `permissions` can then be ignored for this project  | [optional][default to false]
**permissions** | Option<[**Vec<models::OrganizationCustomRoleUpdateRequestProjectPermissionsInnerPermissionsInner>**](OrganizationCustomRoleUpdateRequest_project_permissions_inner_permissions_inner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


