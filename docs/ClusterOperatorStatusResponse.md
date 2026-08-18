# ClusterOperatorStatusResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** |  | 
**cluster_id** | **uuid::Uuid** |  | 
**operator_connected** | **bool** | Whether the last heartbeat is within the Operator presence window. | 
**last_heartbeat** | Option<**String**> |  | [optional]
**operator_version** | Option<**String**> | Display version reported by the Operator. For the POC version-reporting heartbeat, the official chart sets this to the exact installed image tag. Legacy Operators can report opaque build metadata instead. | [optional]
**controller_version** | Option<**String**> |  | [optional]
**request_schema_version** | Option<**String**> |  | [optional]
**desired_image_version** | Option<**String**> | Image tag currently selected for a newly compiled Operator bootstrap. | [optional]
**desired_chart_version** | Option<**String**> | Helm chart version currently selected for a newly compiled Operator bootstrap. | [optional]
**status** | [**models::ClusterOperatorFleetStatus**](ClusterOperatorFleetStatus.md) |  | 
**reported_chart_version** | Option<**String**> | Helm chart version reported by the Operator, even without immutable identity. | [optional]
**reported_identity** | Option<[**models::ReportedClusterOperatorIdentity**](ReportedClusterOperatorIdentity.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


