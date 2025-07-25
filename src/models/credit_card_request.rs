/*
 * Qovery API
 *
 * - Qovery is the fastest way to deploy your full-stack apps on any Cloud provider. - ℹ️ The API is stable and still in development.
 *
 * The version of the OpenAPI document: 1.0.4
 * Contact: support+api+documentation@qovery.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreditCardRequest {
    #[serde(rename = "number")]
    pub number: String,
    #[serde(rename = "cvv")]
    pub cvv: String,
    #[serde(rename = "expiry_month")]
    pub expiry_month: i32,
    #[serde(rename = "expiry_year")]
    pub expiry_year: i32,
}

impl CreditCardRequest {
    pub fn new(
        number: String,
        cvv: String,
        expiry_month: i32,
        expiry_year: i32,
    ) -> CreditCardRequest {
        CreditCardRequest {
            number,
            cvv,
            expiry_month,
            expiry_year,
        }
    }
}
