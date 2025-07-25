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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "port_type")]
pub enum HelmResponseAllOfPorts {
    #[serde(rename = "SERVICE_NAME")]
    ServiceName(models::HelmPortResponseWithServiceName),
    #[serde(rename = "SERVICE_SELECTORS")]
    ServiceSelectors(models::HelmPortResponseWithServiceSelectors),
}

impl Default for HelmResponseAllOfPorts {
    fn default() -> Self {
        Self::ServiceName(Default::default())
    }
}

pub fn default_port_type_service_name() -> String {
    "SERVICE_NAME".to_string()
}

pub fn default_port_type_service_selectors() -> String {
    "SERVICE_SELECTORS".to_string()
}
