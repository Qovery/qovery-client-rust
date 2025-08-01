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
pub struct ClusterLogs {
    /// log level
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// log date creation
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// log step
    #[serde(rename = "step", skip_serializing_if = "Option::is_none")]
    pub step: Option<Step>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<models::ClusterLogsMessage>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<models::ClusterLogsError>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<models::ClusterLogsDetails>,
}

impl ClusterLogs {
    pub fn new() -> ClusterLogs {
        ClusterLogs {
            r#type: None,
            timestamp: None,
            step: None,
            message: None,
            error: None,
            details: None,
        }
    }
}
/// log step
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Step {
    #[serde(rename = "LoadConfiguration")]
    LoadConfiguration,
    #[serde(rename = "Create")]
    Create,
    #[serde(rename = "Created")]
    Created,
    #[serde(rename = "CreateError")]
    CreateError,
    #[serde(rename = "Pause")]
    Pause,
    #[serde(rename = "Paused")]
    Paused,
    #[serde(rename = "PauseError")]
    PauseError,
    #[serde(rename = "Delete")]
    Delete,
    #[serde(rename = "Deleted")]
    Deleted,
    #[serde(rename = "DeleteError")]
    DeleteError,
    #[serde(rename = "RetrieveClusterConfig")]
    RetrieveClusterConfig,
    #[serde(rename = "RetrieveClusterResources")]
    RetrieveClusterResources,
    #[serde(rename = "ValidateSystemRequirements")]
    ValidateSystemRequirements,
    #[serde(rename = "UnderMigration")]
    UnderMigration,
    #[serde(rename = "Unknown")]
    Unknown,
}

impl Default for Step {
    fn default() -> Step {
        Self::LoadConfiguration
    }
}
