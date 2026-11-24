# OrganizationPolicyApiTokenCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**opa_policy** | **String** | Open Policy Agent (rego) rule definitions, without a `package` declaration: Qovery prepends a per-token package so that one token's rules cannot authorize another's. The policy must define an `allow` rule, and the request is denied unless it evaluates to true. | 
**role_id** | Option<**uuid::Uuid**> | The organization role the token acts as: a default role id or a custom role id from the \"List organization custom roles\" endpoint. The role bounds what the token may do once its policy has allowed a request, so effective access is the intersection of the two, and a request the role does not permit is refused with 403. Omit it, or send null, for the organization admin role. The Owner role is refused. The role cannot be changed after creation. | [optional]
**expires_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


