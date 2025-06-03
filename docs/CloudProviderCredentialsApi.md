# \CloudProviderCredentialsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_aws_credentials**](CloudProviderCredentialsApi.md#create_aws_credentials) | **POST** /organization/{organizationId}/aws/credentials | Create AWS credentials set
[**create_azure_credentials**](CloudProviderCredentialsApi.md#create_azure_credentials) | **POST** /organization/{organizationId}/azure/credentials | Create Azure credentials set
[**create_gcp_credentials**](CloudProviderCredentialsApi.md#create_gcp_credentials) | **POST** /organization/{organizationId}/gcp/credentials | Create GCP credentials set
[**create_on_premise_credentials**](CloudProviderCredentialsApi.md#create_on_premise_credentials) | **POST** /organization/{organizationId}/onPremise/credentials | Create OnPremise credentials set
[**create_scaleway_credentials**](CloudProviderCredentialsApi.md#create_scaleway_credentials) | **POST** /organization/{organizationId}/scaleway/credentials | Create Scaleway credentials set
[**delete_aws_credentials**](CloudProviderCredentialsApi.md#delete_aws_credentials) | **DELETE** /organization/{organizationId}/aws/credentials/{credentialsId} | Delete a set of AWS credentials
[**delete_azure_credentials**](CloudProviderCredentialsApi.md#delete_azure_credentials) | **DELETE** /organization/{organizationId}/azure/credentials/{credentialsId} | Delete a set of Azure credentials
[**delete_gcp_credentials**](CloudProviderCredentialsApi.md#delete_gcp_credentials) | **DELETE** /organization/{organizationId}/gcp/credentials/{credentialsId} | Delete a set of GCP credentials
[**delete_on_premise_credentials**](CloudProviderCredentialsApi.md#delete_on_premise_credentials) | **DELETE** /organization/{organizationId}/onPremise/credentials/{credentialsId} | Delete a set of OnPremise credentials
[**delete_scaleway_credentials**](CloudProviderCredentialsApi.md#delete_scaleway_credentials) | **DELETE** /organization/{organizationId}/scaleway/credentials/{credentialsId} | Delete a set of Scaleway credentials
[**edit_aws_credentials**](CloudProviderCredentialsApi.md#edit_aws_credentials) | **PUT** /organization/{organizationId}/aws/credentials/{credentialsId} | Edit a set of AWS credentials
[**edit_azure_credentials**](CloudProviderCredentialsApi.md#edit_azure_credentials) | **PUT** /organization/{organizationId}/azure/credentials/{credentialsId} | Edit a set of Azure credentials
[**edit_gcp_credentials**](CloudProviderCredentialsApi.md#edit_gcp_credentials) | **PUT** /organization/{organizationId}/gcp/credentials/{credentialsId} | Edit a set of GCP credentials
[**edit_on_premise_credentials**](CloudProviderCredentialsApi.md#edit_on_premise_credentials) | **PUT** /organization/{organizationId}/onPremise/credentials/{credentialsId} | Edit a set of OnPremise credentials
[**edit_scaleway_credentials**](CloudProviderCredentialsApi.md#edit_scaleway_credentials) | **PUT** /organization/{organizationId}/scaleway/credentials/{credentialsId} | Edit a set of Scaleway credentials
[**get_aws_credentials**](CloudProviderCredentialsApi.md#get_aws_credentials) | **GET** /organization/{organizationId}/aws/credentials/{credentialsId} | Get a set of AWS credentials
[**get_azure_credentials**](CloudProviderCredentialsApi.md#get_azure_credentials) | **GET** /organization/{organizationId}/azure/credentials/{credentialsId} | Get a set of Azure credentials
[**get_gcp_credentials**](CloudProviderCredentialsApi.md#get_gcp_credentials) | **GET** /organization/{organizationId}/gcp/credentials/{credentialsId} | Get a set of GCP credentials
[**get_on_premise_credentials**](CloudProviderCredentialsApi.md#get_on_premise_credentials) | **GET** /organization/{organizationId}/onPremise/credentials/{credentialsId} | Get a set of OnPremise credentials
[**get_scaleway_credentials**](CloudProviderCredentialsApi.md#get_scaleway_credentials) | **GET** /organization/{organizationId}/scaleway/credentials/{credentialsId} | Get a set of Scaleway credentials
[**list_aws_credentials**](CloudProviderCredentialsApi.md#list_aws_credentials) | **GET** /organization/{organizationId}/aws/credentials | List AWS credentials
[**list_azure_credentials**](CloudProviderCredentialsApi.md#list_azure_credentials) | **GET** /organization/{organizationId}/azure/credentials | List Azure credentials
[**list_gcp_credentials**](CloudProviderCredentialsApi.md#list_gcp_credentials) | **GET** /organization/{organizationId}/gcp/credentials | List GCP credentials
[**list_on_premise_credentials**](CloudProviderCredentialsApi.md#list_on_premise_credentials) | **GET** /organization/{organizationId}/onPremise/credentials | List OnPremise credentials
[**list_scaleway_credentials**](CloudProviderCredentialsApi.md#list_scaleway_credentials) | **GET** /organization/{organizationId}/scaleway/credentials | List Scaleway credentials



## create_aws_credentials

> models::ClusterCredentials create_aws_credentials(organization_id, aws_credentials_request)
Create AWS credentials set

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**aws_credentials_request** | Option<[**AwsCredentialsRequest**](AwsCredentialsRequest.md)> |  |  |

### Return type

[**models::ClusterCredentials**](ClusterCredentials.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_azure_credentials

> models::ClusterCredentials create_azure_credentials(organization_id, azure_credentials_request)
Create Azure credentials set

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**azure_credentials_request** | Option<[**AzureCredentialsRequest**](AzureCredentialsRequest.md)> |  |  |

### Return type

[**models::ClusterCredentials**](ClusterCredentials.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_gcp_credentials

> models::ClusterCredentials create_gcp_credentials(organization_id, gcp_credentials_request)
Create GCP credentials set

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**gcp_credentials_request** | Option<[**GcpCredentialsRequest**](GcpCredentialsRequest.md)> |  |  |

### Return type

[**models::ClusterCredentials**](ClusterCredentials.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_on_premise_credentials

> models::ClusterCredentials create_on_premise_credentials(organization_id, on_premise_credentials_request)
Create OnPremise credentials set

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**on_premise_credentials_request** | Option<[**OnPremiseCredentialsRequest**](OnPremiseCredentialsRequest.md)> |  |  |

### Return type

[**models::ClusterCredentials**](ClusterCredentials.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_scaleway_credentials

> models::ClusterCredentials create_scaleway_credentials(organization_id, scaleway_credentials_request)
Create Scaleway credentials set

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**scaleway_credentials_request** | Option<[**ScalewayCredentialsRequest**](ScalewayCredentialsRequest.md)> |  |  |

### Return type

[**models::ClusterCredentials**](ClusterCredentials.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_aws_credentials

> delete_aws_credentials(credentials_id, organization_id)
Delete a set of AWS credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credentials_id** | **uuid::Uuid** | Credentials ID | [required] |
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_azure_credentials

> delete_azure_credentials(organization_id, credentials_id)
Delete a set of Azure credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** |  | [required] |
**credentials_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_gcp_credentials

> delete_gcp_credentials(credentials_id, organization_id)
Delete a set of GCP credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credentials_id** | **uuid::Uuid** | Credentials ID | [required] |
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_on_premise_credentials

> delete_on_premise_credentials(credentials_id, organization_id)
Delete a set of OnPremise credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credentials_id** | **uuid::Uuid** | Credentials ID | [required] |
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_scaleway_credentials

> delete_scaleway_credentials(credentials_id, organization_id)
Delete a set of Scaleway credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credentials_id** | **uuid::Uuid** | Credentials ID | [required] |
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_aws_credentials

> models::ClusterCredentials edit_aws_credentials(organization_id, credentials_id, aws_credentials_request)
Edit a set of AWS credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**credentials_id** | **uuid::Uuid** | Credentials ID | [required] |
**aws_credentials_request** | Option<[**AwsCredentialsRequest**](AwsCredentialsRequest.md)> |  |  |

### Return type

[**models::ClusterCredentials**](ClusterCredentials.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_azure_credentials

> models::ClusterCredentials edit_azure_credentials(organization_id, credentials_id, azure_credentials_request)
Edit a set of Azure credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** |  | [required] |
**credentials_id** | **uuid::Uuid** |  | [required] |
**azure_credentials_request** | Option<[**AzureCredentialsRequest**](AzureCredentialsRequest.md)> |  |  |

### Return type

[**models::ClusterCredentials**](ClusterCredentials.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_gcp_credentials

> models::ClusterCredentials edit_gcp_credentials(organization_id, credentials_id, gcp_credentials_request)
Edit a set of GCP credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**credentials_id** | **uuid::Uuid** | Credentials ID | [required] |
**gcp_credentials_request** | Option<[**GcpCredentialsRequest**](GcpCredentialsRequest.md)> |  |  |

### Return type

[**models::ClusterCredentials**](ClusterCredentials.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_on_premise_credentials

> models::ClusterCredentials edit_on_premise_credentials(organization_id, credentials_id, on_premise_credentials_request)
Edit a set of OnPremise credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**credentials_id** | **uuid::Uuid** | Credentials ID | [required] |
**on_premise_credentials_request** | Option<[**OnPremiseCredentialsRequest**](OnPremiseCredentialsRequest.md)> |  |  |

### Return type

[**models::ClusterCredentials**](ClusterCredentials.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_scaleway_credentials

> models::ClusterCredentials edit_scaleway_credentials(organization_id, credentials_id, scaleway_credentials_request)
Edit a set of Scaleway credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**credentials_id** | **uuid::Uuid** | Credentials ID | [required] |
**scaleway_credentials_request** | Option<[**ScalewayCredentialsRequest**](ScalewayCredentialsRequest.md)> |  |  |

### Return type

[**models::ClusterCredentials**](ClusterCredentials.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_aws_credentials

> models::ClusterCredentials get_aws_credentials(organization_id, credentials_id)
Get a set of AWS credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**credentials_id** | **uuid::Uuid** | Credentials ID | [required] |

### Return type

[**models::ClusterCredentials**](ClusterCredentials.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_azure_credentials

> models::ClusterCredentials get_azure_credentials(organization_id, credentials_id)
Get a set of Azure credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** |  | [required] |
**credentials_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ClusterCredentials**](ClusterCredentials.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gcp_credentials

> models::ClusterCredentials get_gcp_credentials(organization_id, credentials_id)
Get a set of GCP credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**credentials_id** | **uuid::Uuid** | Credentials ID | [required] |

### Return type

[**models::ClusterCredentials**](ClusterCredentials.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_on_premise_credentials

> models::ClusterCredentials get_on_premise_credentials(organization_id, credentials_id)
Get a set of OnPremise credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**credentials_id** | **uuid::Uuid** | Credentials ID | [required] |

### Return type

[**models::ClusterCredentials**](ClusterCredentials.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scaleway_credentials

> models::ClusterCredentials get_scaleway_credentials(organization_id, credentials_id)
Get a set of Scaleway credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |
**credentials_id** | **uuid::Uuid** | Credentials ID | [required] |

### Return type

[**models::ClusterCredentials**](ClusterCredentials.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_aws_credentials

> models::ClusterCredentialsResponseList list_aws_credentials(organization_id)
List AWS credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::ClusterCredentialsResponseList**](ClusterCredentialsResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_azure_credentials

> models::ClusterCredentialsResponseList list_azure_credentials(organization_id)
List Azure credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::ClusterCredentialsResponseList**](ClusterCredentialsResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gcp_credentials

> models::ClusterCredentialsResponseList list_gcp_credentials(organization_id)
List GCP credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::ClusterCredentialsResponseList**](ClusterCredentialsResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_on_premise_credentials

> models::ClusterCredentialsResponseList list_on_premise_credentials(organization_id)
List OnPremise credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::ClusterCredentialsResponseList**](ClusterCredentialsResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_scaleway_credentials

> models::ClusterCredentialsResponseList list_scaleway_credentials(organization_id)
List Scaleway credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization ID | [required] |

### Return type

[**models::ClusterCredentialsResponseList**](ClusterCredentialsResponseList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

