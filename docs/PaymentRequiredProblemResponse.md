# PaymentRequiredProblemResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Always sent, `about:blank`. | [optional]
**title** | **String** | Always sent, `Payment Required`. | 
**status** | Option<**i32**> | Always sent, 402. | [optional]
**detail** | **String** | Always sent. Plain text to show to the user, saying why the action is refused and what to do: the pricing explanation, or a generic text when there is none. | 
**instance** | Option<**String**> | Always sent. The path of the request. | [optional]
**message** | Option<**String**> | The same text as `detail`. Absent when the refusal carries no pricing explanation. | [optional]
**sku** | Option<**String**> | The plan item that refused the action. Absent when unknown. New items can appear: do not reject an unknown value. | [optional]
**action** | Option<**String**> | The checked action: `member.invite`, `cluster.create`, `cluster.create_self_managed` or `environment.create` (also used for a clone). Absent when unknown. New actions can appear: do not reject an unknown value. | [optional]
**limit** | Option<[**models::PaymentRequiredProblemResponseLimit**](PaymentRequiredProblemResponseLimit.md)> |  | [optional]
**used** | Option<**i32**> | How many of `sku` the organization uses now. Absent when unknown. | [optional]
**requested** | Option<**i32**> | How many of `sku` the action adds. Absent when unknown. | [optional]
**upgrade_url** | Option<**String**> | The console page where a billing role changes the plan, the spend limit or the payment method. Always an `https` URL. Absent when there is no page to send the user to. | [optional]
**offers** | Option<**Vec<std::collections::HashMap<String, serde_json::Value>>**> | Ways past the refusal, produced by the pricing service, in the order to show them. Each offer has a `type` (`plan_change`, `addon`, `contact_sales`, `free_up` or `raise_spend_limit`; more can appear) and a `name`; its other properties depend on the type and can change. Absent unless the caller can manage the organization's billing. | [optional]
**paywall** | Option<**std::collections::HashMap<String, serde_json::Value>**> | What the refused feature does and the ways to get it, produced by the pricing service for a billing role; its properties can change. Absent unless the caller can manage the organization's billing, and absent when the refusal comes from a spend limit. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


