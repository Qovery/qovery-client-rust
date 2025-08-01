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
pub struct OrganizationAnnotationsGroupCreateRequest {
    /// name of the annotations group
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "annotations")]
    pub annotations: Vec<models::Annotation>,
    #[serde(rename = "scopes")]
    pub scopes: Vec<models::OrganizationAnnotationsGroupScopeEnum>,
}

impl OrganizationAnnotationsGroupCreateRequest {
    pub fn new(
        name: String,
        annotations: Vec<models::Annotation>,
        scopes: Vec<models::OrganizationAnnotationsGroupScopeEnum>,
    ) -> OrganizationAnnotationsGroupCreateRequest {
        OrganizationAnnotationsGroupCreateRequest {
            name,
            annotations,
            scopes,
        }
    }
}
