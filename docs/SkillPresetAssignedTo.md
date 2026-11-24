# SkillPresetAssignedTo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_default** | Option<**bool**> | Whether the preset applies to every member with no member or role assignment of their own. | [optional][default to false]
**role_ids** | Option<**Vec<uuid::Uuid>**> | Default or custom roles of the organization, as returned by \"List organization roles\". | [optional][default to []]
**user_ids** | Option<**Vec<String>**> | Members of the organization, by the id returned by \"List organization members\". | [optional][default to []]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


