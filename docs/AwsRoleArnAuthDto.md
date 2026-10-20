# AwsRoleArnAuthDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mode** | **Mode** |  (enum: AWS_ROLE_ARN) | 
**role_arn** | **String** |  | 
**assume_role** | Option<[**models::AwsAssumeRole**](AwsAssumeRole.md)> | Role that External Secrets Operator assumes with sts:AssumeRole after authenticating with role_arn, typically to read secrets stored in another AWS account. The trust policy of this role must allow role_arn, and role_arn needs sts:AssumeRole on it. Only available with AWS_ROLE_ARN authentication. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


