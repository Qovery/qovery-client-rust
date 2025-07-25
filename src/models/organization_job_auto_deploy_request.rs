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
pub struct OrganizationJobAutoDeployRequest {
    /// the job image name to deploy
    #[serde(rename = "image_name", skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    /// the new tag to deploy
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl OrganizationJobAutoDeployRequest {
    pub fn new() -> OrganizationJobAutoDeployRequest {
        OrganizationJobAutoDeployRequest {
            image_name: None,
            tag: None,
        }
    }
}
