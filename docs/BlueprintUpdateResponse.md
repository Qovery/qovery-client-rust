# BlueprintUpdateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_up_to_date** | **bool** |  | 
**current_tag** | **String** |  | 
**latest_tag** | **String** |  | 
**new_required_values** | [**Vec<models::BlueprintUpdateNewRequiredValue>**](BlueprintUpdateNewRequiredValue.md) | Variables added in the latest version that are required with no default | 
**new_optional_values** | [**Vec<models::BlueprintUpdateNewOptionalValue>**](BlueprintUpdateNewOptionalValue.md) | Variables added in the latest version that have a default value | 
**now_required_values** | [**Vec<models::BlueprintUpdateNewRequiredValue>**](BlueprintUpdateNewRequiredValue.md) | Variables that were optional but are now required in the latest version | 
**updated_values** | [**Vec<models::BlueprintUpdateUpdatedValue>**](BlueprintUpdateUpdatedValue.md) | Variables whose default value changed between the current and latest versions | 
**removed_values** | [**Vec<models::BlueprintUpdateRemovedValue>**](BlueprintUpdateRemovedValue.md) | Variables that no longer exist in the latest version | 
**engine_diff** | [**models::BlueprintUpdateEngineDiff**](BlueprintUpdateEngineDiff.md) |  | 
**new_major_versions** | [**Vec<models::BlueprintUpdateNewMajorVersion>**](BlueprintUpdateNewMajorVersion.md) | Major versions of the same blueprint newer than the service's current one (e.g. service on aws/postgres/16 while 17 and 18 exist). Empty when already on the latest major or when the current version is non-numeric. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


