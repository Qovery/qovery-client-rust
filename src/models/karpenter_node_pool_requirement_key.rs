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
pub enum KarpenterNodePoolRequirementKey {
    #[serde(rename = "InstanceFamily")]
    InstanceFamily,
    #[serde(rename = "InstanceSize")]
    InstanceSize,
    #[serde(rename = "Arch")]
    Arch,
    #[serde(rename = "SkuFamily")]
    SkuFamily,
    #[serde(rename = "SkuVersion")]
    SkuVersion,
}

impl std::fmt::Display for KarpenterNodePoolRequirementKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InstanceFamily => write!(f, "InstanceFamily"),
            Self::InstanceSize => write!(f, "InstanceSize"),
            Self::Arch => write!(f, "Arch"),
            Self::SkuFamily => write!(f, "SkuFamily"),
            Self::SkuVersion => write!(f, "SkuVersion"),
        }
    }
}

impl Default for KarpenterNodePoolRequirementKey {
    fn default() -> KarpenterNodePoolRequirementKey {
        Self::InstanceFamily
    }
}
