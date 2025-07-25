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
pub struct HelmDeployRequest {
    /// version of the chart to deploy. Cannot be set if `git_commit_id` is defined
    #[serde(rename = "chart_version", skip_serializing_if = "Option::is_none")]
    pub chart_version: Option<String>,
    /// Commit to deploy for chart source. Cannot be set if `version` is defined
    #[serde(rename = "git_commit_id", skip_serializing_if = "Option::is_none")]
    pub git_commit_id: Option<String>,
    /// Commit to deploy for values override
    #[serde(
        rename = "values_override_git_commit_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub values_override_git_commit_id: Option<String>,
}

impl HelmDeployRequest {
    pub fn new() -> HelmDeployRequest {
        HelmDeployRequest {
            chart_version: None,
            git_commit_id: None,
            values_override_git_commit_id: None,
        }
    }
}
