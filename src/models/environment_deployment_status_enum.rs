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
pub enum EnvironmentDeploymentStatusEnum {
    #[serde(rename = "NEVER_DEPLOYED")]
    NeverDeployed,
    #[serde(rename = "UP_TO_DATE")]
    UpToDate,
    #[serde(rename = "OUT_OF_DATE")]
    OutOfDate,
}

impl std::fmt::Display for EnvironmentDeploymentStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NeverDeployed => write!(f, "NEVER_DEPLOYED"),
            Self::UpToDate => write!(f, "UP_TO_DATE"),
            Self::OutOfDate => write!(f, "OUT_OF_DATE"),
        }
    }
}

impl Default for EnvironmentDeploymentStatusEnum {
    fn default() -> EnvironmentDeploymentStatusEnum {
        Self::NeverDeployed
    }
}
