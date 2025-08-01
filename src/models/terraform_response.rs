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

/// TerraformResponse : A Terraform service
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TerraformResponse {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// name is case insensitive
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "timeout_sec")]
    pub timeout_sec: i32,
    #[serde(rename = "auto_approve")]
    pub auto_approve: bool,
    #[serde(rename = "auto_deploy")]
    pub auto_deploy: bool,
    #[serde(
        rename = "terraform_files_source",
        skip_serializing_if = "Option::is_none"
    )]
    pub terraform_files_source: Option<models::TerraformFilesSource>,
    /// Icon URI representing the terraform service.
    #[serde(rename = "icon_uri")]
    pub icon_uri: String,
    #[serde(
        rename = "service_type",
        default = "models::service_type_enum::service_type_terraform"
    )]
    pub service_type: models::ServiceTypeEnum,
    #[serde(rename = "terraform_variables_source")]
    pub terraform_variables_source: models::TerraformVariablesSourceResponse,
    #[serde(rename = "provider")]
    pub provider: Provider,
    #[serde(rename = "provider_version")]
    pub provider_version: models::TerraformProviderVersion,
    #[serde(rename = "job_resources")]
    pub job_resources: models::TerraformJobResourcesResponse,
    #[serde(rename = "environment")]
    pub environment: models::ReferenceObject,
    #[serde(rename = "use_cluster_credentials")]
    pub use_cluster_credentials: bool,
}

impl TerraformResponse {
    /// A Terraform service
    pub fn new(
        id: uuid::Uuid,
        created_at: String,
        name: String,
        timeout_sec: i32,
        auto_approve: bool,
        auto_deploy: bool,
        icon_uri: String,
        service_type: models::ServiceTypeEnum,
        terraform_variables_source: models::TerraformVariablesSourceResponse,
        provider: Provider,
        provider_version: models::TerraformProviderVersion,
        job_resources: models::TerraformJobResourcesResponse,
        environment: models::ReferenceObject,
        use_cluster_credentials: bool,
    ) -> TerraformResponse {
        TerraformResponse {
            id,
            created_at,
            updated_at: None,
            name,
            description: None,
            timeout_sec,
            auto_approve,
            auto_deploy,
            terraform_files_source: None,
            icon_uri,
            service_type,
            terraform_variables_source,
            provider,
            provider_version,
            job_resources,
            environment,
            use_cluster_credentials,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "TERRAFORM")]
    Terraform,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::Terraform
    }
}
