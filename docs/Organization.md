# Organization

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | **String** | name is case insensitive | 
**description** | Option<**String**> |  | [optional]
**plan** | [**models::PlanEnum**](PlanEnum.md) |  | 
**website_url** | Option<**String**> |  | [optional]
**repository** | Option<**String**> |  | [optional]
**logo_url** | Option<**String**> |  | [optional]
**icon_url** | Option<**String**> |  | [optional]
**admin_emails** | Option<**Vec<String>**> |  | [optional]
**owner** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | uuid of the user owning the organization | [optional]
**billing_deployment_restriction** | Option<**String**> | If set, indicates a billing-related restriction on the organization. 'NO_CREDIT_CARD' means the organization is on a free trial without a credit card — managed cluster creation and deployments on managed clusters are blocked, but demo cluster usage is allowed. Any other value blocks all deployments. null means no restriction. | [optional]
**organization_plan** | Option<[**models::OrganizationAllOfOrganizationPlan**](Organization_allOf_organization_plan.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


