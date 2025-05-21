# \ReferralRewardsApi

All URIs are relative to *https://api.qovery.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_account_referral**](ReferralRewardsApi.md#get_account_referral) | **GET** /account/referral | Get your referral information
[**post_account_reward_claim**](ReferralRewardsApi.md#post_account_reward_claim) | **POST** /account/rewardClaim | Claim a reward



## get_account_referral

> models::Referral get_account_referral()
Get your referral information

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Referral**](Referral.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_account_reward_claim

> post_account_reward_claim(reward_claim)
Claim a reward

A same code can be claimed only 3 times at max

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reward_claim** | Option<[**RewardClaim**](RewardClaim.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

