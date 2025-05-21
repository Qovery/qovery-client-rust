# \BillingApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_credit_card**](BillingApi.md#add_credit_card) | **POST** /organization/{organizationId}/creditCard | Add credit card
[**add_credit_code**](BillingApi.md#add_credit_code) | **POST** /organization/{organizationId}/creditCode | Add credit code
[**change_plan**](BillingApi.md#change_plan) | **POST** /organization/{organizationId}/changePlan | Change organization plan
[**delete_credit_card**](BillingApi.md#delete_credit_card) | **DELETE** /organization/{organizationId}/creditCard/{creditCardId} | Delete credit card
[**edit_organization_billing_info**](BillingApi.md#edit_organization_billing_info) | **PUT** /organization/{organizationId}/billingInfo | Edit Organization Billing Info
[**generate_billing_usage_report**](BillingApi.md#generate_billing_usage_report) | **POST** /organization/{organizationId}/billingUsageReport | Generate organization billing usage report
[**get_cluster_current_cost**](BillingApi.md#get_cluster_current_cost) | **GET** /organization/{organizationId}/cluster/{clusterId}/currentCost | Get cluster current cost
[**get_organization_billing_external_id**](BillingApi.md#get_organization_billing_external_id) | **GET** /organization/{organizationId}/billingExternalId | Get organization billing external ID
[**get_organization_billing_info**](BillingApi.md#get_organization_billing_info) | **GET** /organization/{organizationId}/billingInfo | Get organization billing info
[**get_organization_billing_status**](BillingApi.md#get_organization_billing_status) | **GET** /organization/{organizationId}/billingStatus | Get organization billing status
[**get_organization_current_cost**](BillingApi.md#get_organization_current_cost) | **GET** /organization/{organizationId}/currentCost | Get organization current cost
[**get_organization_invoice**](BillingApi.md#get_organization_invoice) | **GET** /organization/{organizationId}/invoice/{invoiceId} | Get organization invoice
[**get_organization_invoice_pdf**](BillingApi.md#get_organization_invoice_pdf) | **GET** /organization/{organizationId}/invoice/{invoiceId}/download | Get invoice link
[**list_organization_credit_cards**](BillingApi.md#list_organization_credit_cards) | **GET** /organization/{organizationId}/creditCard | List organization credit cards
[**list_organization_invoice**](BillingApi.md#list_organization_invoice) | **GET** /organization/{organizationId}/invoice | List organization invoices
[**organization_download_all_invoices**](BillingApi.md#organization_download_all_invoices) | **POST** /organization/{organizationId}/downloadInvoices | Download all invoices



## add_credit_card

> models::CreditCard add_credit_card(organization_id, credit_card_request)
Add credit card

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**credit_card_request** | Option<[**CreditCardRequest**](CreditCardRequest.md)> |  |  |

### Return type

[**models::CreditCard**](CreditCard.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_credit_code

> add_credit_code(organization_id, organization_credit_code_request)
Add credit code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**organization_credit_code_request** | Option<[**OrganizationCreditCodeRequest**](OrganizationCreditCodeRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_plan

> models::Organization change_plan(organization_id, organization_change_plan_request)
Change organization plan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**organization_change_plan_request** | Option<[**OrganizationChangePlanRequest**](OrganizationChangePlanRequest.md)> |  |  |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_credit_card

> delete_credit_card(organization_id, credit_card_id)
Delete credit card

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**credit_card_id** | **uuid::Uuid** | Credit Card ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_organization_billing_info

> models::BillingInfo edit_organization_billing_info(organization_id, billing_info_request)
Edit Organization Billing Info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**billing_info_request** | Option<[**BillingInfoRequest**](BillingInfoRequest.md)> |  |  |

### Return type

[**models::BillingInfo**](BillingInfo.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_billing_usage_report

> models::OrganizationBillingUsageReportResponse generate_billing_usage_report(organization_id, organization_billing_usage_report_request)
Generate organization billing usage report

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**organization_billing_usage_report_request** | Option<[**OrganizationBillingUsageReportRequest**](OrganizationBillingUsageReportRequest.md)> |  |  |

### Return type

[**models::OrganizationBillingUsageReportResponse**](OrganizationBillingUsageReportResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_current_cost

> models::CostRange get_cluster_current_cost(organization_id, cluster_id)
Get cluster current cost

Get your cluster cost range. We are unable to give a precise cost of your infrastructure at the moment. But Qovery guarantees that the cost of your cluster will not exceed the max range. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**cluster_id** | **uuid::Uuid** | Cluster ID | [required] |

### Return type

[**models::CostRange**](CostRange.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_billing_external_id

> models::BillingExternalId get_organization_billing_external_id(organization_id)
Get organization billing external ID

This endpoint returns the external ID of the organization's billing account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::BillingExternalId**](BillingExternalId.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_billing_info

> models::BillingInfo get_organization_billing_info(organization_id)
Get organization billing info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::BillingInfo**](BillingInfo.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_billing_status

> models::BillingStatus get_organization_billing_status(organization_id)
Get organization billing status

This endpoint returns a \"is_valid\" boolean field reflecting the billing status of the organization: - If true, the organization billing is valid - For Startup organization, it returns false if there is at least 1 invoice unpaid since 1 week - For Community organization, it returns false if there is no credit left 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::BillingStatus**](BillingStatus.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_current_cost

> models::OrganizationCurrentCost get_organization_current_cost(organization_id)
Get organization current cost

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::OrganizationCurrentCost**](OrganizationCurrentCost.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_invoice

> models::Invoice get_organization_invoice(organization_id, invoice_id)
Get organization invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**invoice_id** | **uuid::Uuid** | Invoice ID | [required] |

### Return type

[**models::Invoice**](Invoice.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_invoice_pdf

> models::Link get_organization_invoice_pdf(organization_id, invoice_id)
Get invoice link

This will return URL of the invoice PDF

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**invoice_id** | **uuid::Uuid** | Invoice ID | [required] |

### Return type

[**models::Link**](Link.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_credit_cards

> models::CreditCardResponseList list_organization_credit_cards(organization_id)
List organization credit cards

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::CreditCardResponseList**](CreditCardResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_organization_invoice

> models::InvoiceResponseList list_organization_invoice(organization_id)
List organization invoices

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::InvoiceResponseList**](InvoiceResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organization_download_all_invoices

> organization_download_all_invoices(organization_id)
Download all invoices

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

