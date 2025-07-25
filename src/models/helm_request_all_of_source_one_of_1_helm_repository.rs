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
pub struct HelmRequestAllOfSourceOneOf1HelmRepository {
    /// The id of the helm repository
    #[serde(
        rename = "repository",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub repository: Option<Option<uuid::Uuid>>,
    /// The name of the chart in the repository
    #[serde(rename = "chart_name", skip_serializing_if = "Option::is_none")]
    pub chart_name: Option<String>,
    /// The version of the chart to use
    #[serde(rename = "chart_version", skip_serializing_if = "Option::is_none")]
    pub chart_version: Option<String>,
}

impl HelmRequestAllOfSourceOneOf1HelmRepository {
    pub fn new() -> HelmRequestAllOfSourceOneOf1HelmRepository {
        HelmRequestAllOfSourceOneOf1HelmRepository {
            repository: None,
            chart_name: None,
            chart_version: None,
        }
    }
}
