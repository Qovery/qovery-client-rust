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
pub struct ProjectDeploymentRule {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// name is case insensitive
    #[serde(rename = "name")]
    pub name: String,
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
    #[serde(rename = "mode")]
    pub mode: models::EnvironmentModeEnum,
    #[serde(rename = "cluster_id")]
    pub cluster_id: uuid::Uuid,
    #[serde(rename = "auto_stop", skip_serializing_if = "Option::is_none")]
    pub auto_stop: Option<bool>,
    #[serde(rename = "timezone")]
    pub timezone: String,
    #[serde(rename = "start_time")]
    pub start_time: String,
    #[serde(rename = "stop_time")]
    pub stop_time: String,
    #[serde(rename = "weekdays")]
    pub weekdays: Vec<models::WeekdayEnum>,
    /// wildcard pattern composed of '?' and/or '*' used to target new created environments
    #[serde(rename = "wildcard")]
    pub wildcard: String,
    /// used to select the first deployment rule to match new created environments
    #[serde(rename = "priority_index", skip_serializing_if = "Option::is_none")]
    pub priority_index: Option<i32>,
}

impl ProjectDeploymentRule {
    pub fn new(
        id: uuid::Uuid,
        created_at: String,
        name: String,
        mode: models::EnvironmentModeEnum,
        cluster_id: uuid::Uuid,
        timezone: String,
        start_time: String,
        stop_time: String,
        weekdays: Vec<models::WeekdayEnum>,
        wildcard: String,
    ) -> ProjectDeploymentRule {
        ProjectDeploymentRule {
            id,
            created_at,
            updated_at: None,
            name,
            description: None,
            mode,
            cluster_id,
            auto_stop: None,
            timezone,
            start_time,
            stop_time,
            weekdays,
            wildcard,
            priority_index: None,
        }
    }
}
