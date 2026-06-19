# BlueprintUpdateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_up_to_date** | **bool** |  | 
**latest_tag** | **String** |  | 
**new_required_values** | [**Vec<models::BlueprintUpdateNewRequiredValue>**](BlueprintUpdateNewRequiredValue.md) | Variables added in the latest version that are required with no default | 
**new_optional_values** | [**Vec<models::BlueprintUpdateNewOptionalValue>**](BlueprintUpdateNewOptionalValue.md) | Variables added in the latest version that have a default value | 
**now_required_values** | [**Vec<models::BlueprintUpdateNewRequiredValue>**](BlueprintUpdateNewRequiredValue.md) | Variables that were optional but are now required in the latest version | 
**updated_values** | [**Vec<models::BlueprintUpdateUpdatedValue>**](BlueprintUpdateUpdatedValue.md) | Variables whose default value changed between the current and latest versions | 
**removed_values** | [**Vec<models::BlueprintUpdateRemovedValue>**](BlueprintUpdateRemovedValue.md) | Variables that no longer exist in the latest version | 
**engine_diff** | [**models::BlueprintUpdateEngineDiff**](BlueprintUpdateEngineDiff.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


