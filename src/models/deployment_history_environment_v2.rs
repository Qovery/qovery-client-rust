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
pub struct DeploymentHistoryEnvironmentV2 {
    #[serde(rename = "identifier")]
    pub identifier: models::DeploymentHistoryEnvironmentV2Identifier,
    #[serde(rename = "auditing_data")]
    pub auditing_data: models::DeploymentHistoryAuditingData,
    #[serde(rename = "status")]
    pub status: models::StateEnum,
    #[serde(rename = "trigger_action")]
    pub trigger_action: models::DeploymentHistoryTriggerAction,
    #[serde(rename = "total_duration")]
    pub total_duration: String,
    #[serde(rename = "stages")]
    pub stages: Vec<models::DeploymentHistoryStage>,
    #[serde(rename = "action_status")]
    pub action_status: models::DeploymentHistoryActionStatus,
}

impl DeploymentHistoryEnvironmentV2 {
    pub fn new(
        identifier: models::DeploymentHistoryEnvironmentV2Identifier,
        auditing_data: models::DeploymentHistoryAuditingData,
        status: models::StateEnum,
        trigger_action: models::DeploymentHistoryTriggerAction,
        total_duration: String,
        stages: Vec<models::DeploymentHistoryStage>,
        action_status: models::DeploymentHistoryActionStatus,
    ) -> DeploymentHistoryEnvironmentV2 {
        DeploymentHistoryEnvironmentV2 {
            identifier,
            auditing_data,
            status,
            trigger_action,
            total_duration,
            stages,
            action_status,
        }
    }
}
