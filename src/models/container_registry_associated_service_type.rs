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
pub enum ContainerRegistryAssociatedServiceType {
    #[serde(rename = "CONTAINER")]
    Container,
    #[serde(rename = "LIFECYCLE")]
    Lifecycle,
    #[serde(rename = "CRON")]
    Cron,
}

impl std::fmt::Display for ContainerRegistryAssociatedServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Container => write!(f, "CONTAINER"),
            Self::Lifecycle => write!(f, "LIFECYCLE"),
            Self::Cron => write!(f, "CRON"),
        }
    }
}

impl Default for ContainerRegistryAssociatedServiceType {
    fn default() -> ContainerRegistryAssociatedServiceType {
        Self::Container
    }
}
