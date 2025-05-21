# OrganizationCustomRoleUpdateRequestProjectPermissionsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**project_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**is_admin** | Option<**bool**> | If `is_admin` is `true`, the user is: - automatically `MANAGER` for each environment type - allowed to manage project deployment rules - able to delete the project    Note that `permissions` can then be ignored for this project  | [optional][default to false]
**permissions** | Option<[**Vec<models::OrganizationCustomRoleUpdateRequestProjectPermissionsInnerPermissionsInner>**](OrganizationCustomRoleUpdateRequest_project_permissions_inner_permissions_inner.md)> | Mandatory if `is_admin` is `false`   Should contain an entry for every environment type: - `DEVELOPMENT` - `PREVIEW` - `STAGING` - `PRODUCTION`  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


