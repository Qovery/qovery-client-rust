# EnterpriseConnectionDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_role** | **String** | The purpose of this default role is to be associated to your users if: - you choose to not expose your IDPs groups to the SAML / OIDC connection - no associated group is found in your `group_mappings` defined  You can define either a Qovery provided role (i.e `viewer`) or one of your custom role`s uuid.  | 
**group_mappings** | [**std::collections::HashMap<String, Vec<String>>**](Vec.md) | This will allow to create mapping rules based on your IDP group names.   It's a dictionnary having: - key: either a Qovery provided role (i.e `viewer`) or one of your custom role`s uuid - value: an array of your IDP group names  Example: \"I want to associate the Qovery role `devops` to my IDP groups ['Administrators', 'DevSecOps']\"  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


