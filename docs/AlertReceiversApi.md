# \AlertReceiversApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_alert_receiver**](AlertReceiversApi.md#create_alert_receiver) | **POST** /alert-receivers | Create alert receiver
[**delete_alert_receiver**](AlertReceiversApi.md#delete_alert_receiver) | **DELETE** /alert-receivers/{alertReceiverId} | Delete alert receiver
[**edit_alert_receiver**](AlertReceiversApi.md#edit_alert_receiver) | **PUT** /alert-receivers/{alertReceiverId} | Update alert receiver
[**get_alert_receiver**](AlertReceiversApi.md#get_alert_receiver) | **GET** /alert-receivers/{alertReceiverId} | Get alert receiver
[**get_alert_receivers**](AlertReceiversApi.md#get_alert_receivers) | **GET** /organization/{organizationId}/alert-receivers | List alert receivers
[**validate_existing_alert_receiver**](AlertReceiversApi.md#validate_existing_alert_receiver) | **POST** /alert-receivers/{alertReceiverId}/validate | Validate Existing Alert Receiver
[**validate_new_alert_receiver**](AlertReceiversApi.md#validate_new_alert_receiver) | **POST** /alert-receivers/validate | Validate New Alert Receiver



## create_alert_receiver

> models::AlertReceiverResponse create_alert_receiver(alert_receiver_creation_request)
Create alert receiver

Create a new alert receiver

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_receiver_creation_request** | Option<[**AlertReceiverCreationRequest**](AlertReceiverCreationRequest.md)> |  |  |

### Return type

[**models::AlertReceiverResponse**](AlertReceiverResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_alert_receiver

> delete_alert_receiver(alert_receiver_id)
Delete alert receiver

Delete an alert receiver

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_receiver_id** | **uuid::Uuid** | Alert Receiver ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_alert_receiver

> models::AlertReceiverResponse edit_alert_receiver(alert_receiver_id, alert_receiver_edit_request)
Update alert receiver

Update an existing alert receiver

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_receiver_id** | **uuid::Uuid** | Alert Receiver ID | [required] |
**alert_receiver_edit_request** | Option<[**AlertReceiverEditRequest**](AlertReceiverEditRequest.md)> |  |  |

### Return type

[**models::AlertReceiverResponse**](AlertReceiverResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alert_receiver

> models::AlertReceiverResponse get_alert_receiver(alert_receiver_id)
Get alert receiver

Retrieve a specific alert receiver by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_receiver_id** | **uuid::Uuid** | Alert Receiver ID | [required] |

### Return type

[**models::AlertReceiverResponse**](AlertReceiverResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alert_receivers

> models::AlertReceiverList get_alert_receivers(organization_id)
List alert receivers

Retrieve all alert receivers for a specific organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::AlertReceiverList**](AlertReceiverList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_existing_alert_receiver

> validate_existing_alert_receiver(alert_receiver_id, alert_receiver_validation_request)
Validate Existing Alert Receiver

Validate an existing alert receiver by sending a test message for example

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_receiver_id** | **uuid::Uuid** | Alert Receiver ID | [required] |
**alert_receiver_validation_request** | Option<[**AlertReceiverValidationRequest**](AlertReceiverValidationRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_new_alert_receiver

> validate_new_alert_receiver(alert_receiver_creation_validation_request)
Validate New Alert Receiver

Validate a future alert receiver by sending a test message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_receiver_creation_validation_request** | Option<[**AlertReceiverCreationValidationRequest**](AlertReceiverCreationValidationRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

