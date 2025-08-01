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
pub struct QueuedDeploymentRequestWithStagesIdentifier {
    #[serde(rename = "deployment_request_id")]
    pub deployment_request_id: uuid::Uuid,
    #[serde(rename = "environment_id")]
    pub environment_id: uuid::Uuid,
}

impl QueuedDeploymentRequestWithStagesIdentifier {
    pub fn new(
        deployment_request_id: uuid::Uuid,
        environment_id: uuid::Uuid,
    ) -> QueuedDeploymentRequestWithStagesIdentifier {
        QueuedDeploymentRequestWithStagesIdentifier {
            deployment_request_id,
            environment_id,
        }
    }
}
