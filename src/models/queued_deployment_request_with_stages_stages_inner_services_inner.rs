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
pub struct QueuedDeploymentRequestWithStagesStagesInnerServicesInner {
    #[serde(rename = "identifier")]
    pub identifier: models::QueuedDeploymentRequestWithStagesStagesInnerServicesInnerIdentifier,
    #[serde(rename = "status")]
    pub status: models::StageStatusEnum,
    #[serde(rename = "icon_uri", skip_serializing_if = "Option::is_none")]
    pub icon_uri: Option<String>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<models::QueuedDeploymentRequestWithStagesStagesInnerServicesInnerDetails>,
}

impl QueuedDeploymentRequestWithStagesStagesInnerServicesInner {
    pub fn new(
        identifier: models::QueuedDeploymentRequestWithStagesStagesInnerServicesInnerIdentifier,
        status: models::StageStatusEnum,
    ) -> QueuedDeploymentRequestWithStagesStagesInnerServicesInner {
        QueuedDeploymentRequestWithStagesStagesInnerServicesInner {
            identifier,
            status,
            icon_uri: None,
            details: None,
        }
    }
}
