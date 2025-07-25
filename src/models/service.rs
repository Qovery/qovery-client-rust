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
pub struct Service {
    /// uuid of the associated service (application, database, job, gateway...)
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::ServiceTypeEnum>,
    /// name of the service
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Git commit ID corresponding to the deployed version of the application
    #[serde(rename = "deployed_commit_id", skip_serializing_if = "Option::is_none")]
    pub deployed_commit_id: Option<String>,
    /// uuid of the user that made the last update
    #[serde(rename = "last_updated_by", skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<uuid::Uuid>,
    /// global overview of resources consumption of the service
    #[serde(
        rename = "consumed_resources_in_percent",
        skip_serializing_if = "Option::is_none"
    )]
    pub consumed_resources_in_percent: Option<f64>,
    /// describes the typology of service (container, postgresl, redis...)
    #[serde(rename = "service_typology", skip_serializing_if = "Option::is_none")]
    pub service_typology: Option<String>,
    /// for databases this field exposes the database version
    #[serde(rename = "service_version", skip_serializing_if = "Option::is_none")]
    pub service_version: Option<String>,
    #[serde(rename = "to_update", skip_serializing_if = "Option::is_none")]
    pub to_update: Option<bool>,
}

impl Service {
    pub fn new(id: uuid::Uuid, created_at: String) -> Service {
        Service {
            id,
            created_at,
            updated_at: None,
            r#type: None,
            name: None,
            deployed_commit_id: None,
            last_updated_by: None,
            consumed_resources_in_percent: None,
            service_typology: None,
            service_version: None,
            to_update: None,
        }
    }
}
