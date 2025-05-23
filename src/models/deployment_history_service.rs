/*
 * Qovery API
 *
 * - Qovery is the fastest way to deploy your full-stack apps on any Cloud provider. - ℹ️ The API is stable and still in development. 
 *
 * The version of the OpenAPI document: 1.0.3
 * Contact: support+api+documentation@qovery.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentHistoryService {
    #[serde(rename = "identifier")]
    pub identifier: Box<models::DeploymentHistoryServiceIdentifier>,
    #[serde(rename = "status")]
    pub status: models::StateEnum,
    #[serde(rename = "auditing_data")]
    pub auditing_data: Box<models::DeploymentHistoryAuditingData>,
    #[serde(rename = "details")]
    pub details: Box<models::DeploymentHistoryServiceDetails>,
    #[serde(rename = "status_details", skip_serializing_if = "Option::is_none")]
    pub status_details: Option<Box<models::StatusDetails>>,
    #[serde(rename = "icon_uri")]
    pub icon_uri: String,
    #[serde(rename = "total_duration", skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<String>,
}

impl DeploymentHistoryService {
    pub fn new(identifier: models::DeploymentHistoryServiceIdentifier, status: models::StateEnum, auditing_data: models::DeploymentHistoryAuditingData, details: models::DeploymentHistoryServiceDetails, icon_uri: String) -> DeploymentHistoryService {
        DeploymentHistoryService {
            identifier: Box::new(identifier),
            status,
            auditing_data: Box::new(auditing_data),
            details: Box::new(details),
            status_details: None,
            icon_uri,
            total_duration: None,
        }
    }
}

