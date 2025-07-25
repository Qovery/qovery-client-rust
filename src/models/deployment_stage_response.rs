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
pub struct DeploymentStageResponse {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "environment")]
    pub environment: models::ReferenceObject,
    /// name is case insensitive
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Position of the deployment stage within the environment
    #[serde(rename = "deployment_order", skip_serializing_if = "Option::is_none")]
    pub deployment_order: Option<i32>,
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<models::DeploymentStageServiceResponse>>,
}

impl DeploymentStageResponse {
    pub fn new(
        id: uuid::Uuid,
        created_at: String,
        environment: models::ReferenceObject,
    ) -> DeploymentStageResponse {
        DeploymentStageResponse {
            id,
            created_at,
            updated_at: None,
            environment,
            name: None,
            description: None,
            deployment_order: None,
            services: None,
        }
    }
}
