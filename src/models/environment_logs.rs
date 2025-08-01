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
pub struct EnvironmentLogs {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "details")]
    pub details: models::EnvironmentLogsDetails,
    #[serde(
        rename = "error",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub error: Option<Option<models::EnvironmentLogsError>>,
    #[serde(
        rename = "message",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub message: Option<Option<models::EnvironmentLogsMessage>>,
}

impl EnvironmentLogs {
    pub fn new(
        r#type: String,
        timestamp: String,
        details: models::EnvironmentLogsDetails,
    ) -> EnvironmentLogs {
        EnvironmentLogs {
            r#type,
            timestamp,
            details,
            error: None,
            message: None,
        }
    }
}
