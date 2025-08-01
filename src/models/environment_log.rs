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
pub struct EnvironmentLog {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<models::EnvironmentLogScope>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<models::StatusKindEnum>,
    /// Log message
    #[serde(rename = "message", deserialize_with = "Option::deserialize")]
    pub message: Option<String>,
    /// Only for errors. Helps Qovery team to investigate.
    #[serde(rename = "execution_id", skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(rename = "hint", skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}

impl EnvironmentLog {
    pub fn new(id: uuid::Uuid, created_at: String, message: Option<String>) -> EnvironmentLog {
        EnvironmentLog {
            id,
            created_at,
            scope: None,
            state: None,
            message,
            execution_id: None,
            hint: None,
        }
    }
}
