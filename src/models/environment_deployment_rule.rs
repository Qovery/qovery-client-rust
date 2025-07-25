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
pub struct EnvironmentDeploymentRule {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "on_demand_preview", skip_serializing_if = "Option::is_none")]
    pub on_demand_preview: Option<bool>,
    #[serde(rename = "auto_stop", skip_serializing_if = "Option::is_none")]
    pub auto_stop: Option<bool>,
    #[serde(rename = "auto_preview", skip_serializing_if = "Option::is_none")]
    pub auto_preview: Option<bool>,
    #[serde(rename = "timezone")]
    pub timezone: String,
    #[serde(rename = "start_time")]
    pub start_time: String,
    #[serde(rename = "stop_time")]
    pub stop_time: String,
    #[serde(rename = "weekdays")]
    pub weekdays: Vec<models::WeekdayEnum>,
}

impl EnvironmentDeploymentRule {
    pub fn new(
        id: uuid::Uuid,
        created_at: String,
        timezone: String,
        start_time: String,
        stop_time: String,
        weekdays: Vec<models::WeekdayEnum>,
    ) -> EnvironmentDeploymentRule {
        EnvironmentDeploymentRule {
            id,
            created_at,
            updated_at: None,
            on_demand_preview: None,
            auto_stop: None,
            auto_preview: None,
            timezone,
            start_time,
            stop_time,
            weekdays,
        }
    }
}
