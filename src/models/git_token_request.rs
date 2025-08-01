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
pub struct GitTokenRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub r#type: models::GitProviderEnum,
    /// The token from your git provider side
    #[serde(rename = "token")]
    pub token: String,
    /// Mandatory only for BITBUCKET git provider, to allow us to fetch repositories at creation/edition of a service
    #[serde(rename = "workspace", skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
}

impl GitTokenRequest {
    pub fn new(name: String, r#type: models::GitProviderEnum, token: String) -> GitTokenRequest {
        GitTokenRequest {
            name,
            description: None,
            r#type,
            token,
            workspace: None,
        }
    }
}
