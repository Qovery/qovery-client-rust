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
pub struct ClusterLogsErrorEventDetails {
    /// cloud provider used
    #[serde(rename = "provider_kind", skip_serializing_if = "Option::is_none")]
    pub provider_kind: Option<String>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "transmitter", skip_serializing_if = "Option::is_none")]
    pub transmitter: Option<models::ClusterLogsErrorEventDetailsTransmitter>,
}

impl ClusterLogsErrorEventDetails {
    pub fn new() -> ClusterLogsErrorEventDetails {
        ClusterLogsErrorEventDetails {
            provider_kind: None,
            region: None,
            transmitter: None,
        }
    }
}
