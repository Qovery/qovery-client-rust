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
pub struct ServiceStorageRequestStorageInner {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "type")]
    pub r#type: models::StorageTypeEnum,
    /// unit is GB Minimum size is 4 GB
    #[serde(rename = "size")]
    pub size: i32,
    #[serde(rename = "mount_point")]
    pub mount_point: String,
}

impl ServiceStorageRequestStorageInner {
    pub fn new(
        r#type: models::StorageTypeEnum,
        size: i32,
        mount_point: String,
    ) -> ServiceStorageRequestStorageInner {
        ServiceStorageRequestStorageInner {
            id: None,
            r#type,
            size,
            mount_point,
        }
    }
}
