# OrganizationAgentApiTokenCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**opa_policy** | **String** | Open Policy Agent (rego) rule definitions, without a `package` declaration: Qovery prepends a per-token package so that one token's rules cannot authorize another's. The policy must define an `allow` rule, and the request is denied unless it evaluates to true. | 
**expires_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


