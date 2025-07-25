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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreateEnvironmentModeEnum {
    #[serde(rename = "DEVELOPMENT")]
    Development,
    #[serde(rename = "PRODUCTION")]
    Production,
    #[serde(rename = "STAGING")]
    Staging,
}

impl std::fmt::Display for CreateEnvironmentModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Development => write!(f, "DEVELOPMENT"),
            Self::Production => write!(f, "PRODUCTION"),
            Self::Staging => write!(f, "STAGING"),
        }
    }
}

impl Default for CreateEnvironmentModeEnum {
    fn default() -> CreateEnvironmentModeEnum {
        Self::Development
    }
}
