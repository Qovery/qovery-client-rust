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
pub struct ContainerVersionResponse {
    #[serde(rename = "image_name", skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[serde(rename = "versions", skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<String>>,
}

impl ContainerVersionResponse {
    pub fn new() -> ContainerVersionResponse {
        ContainerVersionResponse {
            image_name: None,
            versions: None,
        }
    }
}
