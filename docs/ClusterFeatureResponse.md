# ClusterFeatureResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**title** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**cost_per_month_in_cents** | Option<**i32**> |  | [optional]
**cost_per_month** | Option<**f64**> |  | [optional]
**currency_code** | Option<**String**> |  | [optional]
**is_cloud_provider_paying_feature** | Option<**bool**> |  | [optional]
**cloud_provider_feature_documentation** | Option<**String**> |  | [optional]
**is_qovery_paying_feature** | Option<**bool**> |  | [optional]
**qovery_feature_documentation** | Option<**String**> |  | [optional]
**value_type** | Option<**String**> |  | [optional]
**value_object** | Option<[**models::ClusterFeatureResponseValueObject**](ClusterFeatureResponse_value_object.md)> |  | [optional]
**is_value_updatable** | Option<**bool**> |  | [optional][default to false]
**accepted_values** | Option<[**Vec<models::ClusterFeatureResponseAcceptedValuesInner>**](ClusterFeatureResponse_accepted_values_inner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


