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
pub struct ContainerRegistryResponseAllOfConfig {
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(
        rename = "scaleway_access_key",
        skip_serializing_if = "Option::is_none"
    )]
    pub scaleway_access_key: Option<String>,
    #[serde(
        rename = "scaleway_project_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub scaleway_project_id: Option<String>,
    #[serde(rename = "access_key_id", skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "role_arn", skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "azure_tenant_id", skip_serializing_if = "Option::is_none")]
    pub azure_tenant_id: Option<String>,
    #[serde(
        rename = "azure_subscription_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub azure_subscription_id: Option<String>,
    #[serde(
        rename = "azure_application_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub azure_application_id: Option<String>,
    #[serde(
        rename = "azure_application_object_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub azure_application_object_id: Option<String>,
}

impl ContainerRegistryResponseAllOfConfig {
    pub fn new() -> ContainerRegistryResponseAllOfConfig {
        ContainerRegistryResponseAllOfConfig {
            username: None,
            region: None,
            scaleway_access_key: None,
            scaleway_project_id: None,
            access_key_id: None,
            role_arn: None,
            azure_tenant_id: None,
            azure_subscription_id: None,
            azure_application_id: None,
            azure_application_object_id: None,
        }
    }
}
