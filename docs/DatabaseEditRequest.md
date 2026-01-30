# DatabaseEditRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | name is case-insensitive | [optional]
**description** | Option<**String**> | give a description to this database | [optional]
**version** | Option<**String**> |  | [optional]
**accessibility** | Option<[**models::DatabaseAccessibilityEnum**](DatabaseAccessibilityEnum.md)> |  | [optional]
**cpu** | Option<**i32**> | unit is millicores (m). 1000m = 1 cpu. This field will be ignored for managed DB (instance type will be used instead).  | [optional][default to 250]
**memory** | Option<**i32**> | unit is MB. 1024 MB = 1GB This field will be ignored for managed DB (instance type will be used instead). Default value is linked to the database type: - MANAGED: 100 - CONTAINER   - POSTGRES: 100   - REDIS: 100   - MYSQL: 512   - MONGODB: 256  | [optional]
**storage** | Option<**i32**> | unit is GB | [optional]
**instance_type** | Option<**String**> | Database instance type to be used for this database. The list of values can be retrieved via the endpoint /{CloudProvider}/managedDatabase/instanceType/{region}/{dbType}. This field SHOULD NOT be set for container DB. | [optional]
**annotations_groups** | Option<[**Vec<models::ServiceAnnotationRequest>**](ServiceAnnotationRequest.md)> |  | [optional]
**labels_groups** | Option<[**Vec<models::ServiceLabelRequest>**](ServiceLabelRequest.md)> |  | [optional]
**icon_uri** | Option<**String**> | Icon URI representing the database. | [optional]
**apply_immediately** | Option<**bool**> | Apply changes immediately instead of waiting for the maintenance window. This field is only applicable for managed databases. Warning: Applying changes immediately may cause a brief service interruption.  | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


