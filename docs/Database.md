# Database

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | [readonly]
**created_at** | **String** |  | [readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**name** | **String** | name is case insensitive | 
**description** | Option<**String**> | give a description to this database | [optional]
**r#type** | [**models::DatabaseTypeEnum**](DatabaseTypeEnum.md) |  | 
**version** | **String** |  | 
**mode** | [**models::DatabaseModeEnum**](DatabaseModeEnum.md) |  | 
**accessibility** | Option<[**models::DatabaseAccessibilityEnum**](DatabaseAccessibilityEnum.md)> |  | [optional]
**cpu** | Option<**i32**> | unit is millicores (m). 1000m = 1 cpu This field will be ignored for managed DB (instance type will be used instead).  | [optional][default to 250]
**instance_type** | Option<**String**> | Database instance type to be used for this database. The list of values can be retrieved via the endpoint /{CloudProvider}/managedDatabase/instanceType/{region}/{dbType}. This field is null for container DB. | [optional]
**memory** | Option<**i32**> | unit is MB. 1024 MB = 1GB This field will be ignored for managed DB (instance type will be used instead). Default value is linked to the database type: - MANAGED: `100` - CONTAINER   - POSTGRES: `100`   - REDIS: `100`   - MYSQL: `512`   - MONGODB: `256`  | [optional]
**storage** | Option<**i32**> | unit is GB | [optional][default to 10]
**annotations_groups** | Option<[**Vec<models::OrganizationAnnotationsGroupResponse>**](OrganizationAnnotationsGroupResponse.md)> |  | [optional]
**labels_groups** | Option<[**Vec<models::OrganizationLabelsGroupResponse>**](OrganizationLabelsGroupResponse.md)> |  | [optional]
**icon_uri** | **String** | Icon URI representing the database. | 
**environment** | [**models::ReferenceObject**](ReferenceObject.md) |  | 
**host** | Option<**String**> |  | [optional]
**port** | Option<**i32**> |  | [optional]
**maximum_cpu** | Option<**i32**> | Maximum cpu that can be allocated to the database based on organization cluster configuration. unit is millicores (m). 1000m = 1 cpu | [optional]
**maximum_memory** | Option<**i32**> | Maximum memory that can be allocated to the database based on organization cluster configuration. unit is MB. 1024 MB = 1GB | [optional]
**disk_encrypted** | Option<**bool**> | indicates if the database disk is encrypted or not | [optional]
**apply_immediately** | Option<**bool**> | Apply changes immediately instead of waiting for the maintenance window. This field is only applicable for managed databases.  | [optional][default to false]
**service_type** | [**models::ServiceTypeEnum**](ServiceTypeEnum.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


