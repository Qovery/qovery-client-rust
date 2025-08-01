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
pub struct VariableAliasRequest {
    /// the value to be used as Alias of the targeted environment variable.
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "alias_scope")]
    pub alias_scope: models::ApiVariableScopeEnum,
    /// the id of the variable that is aliased.
    #[serde(rename = "alias_parent_id")]
    pub alias_parent_id: uuid::Uuid,
    /// optional variable description (255 characters maximum)
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
    #[serde(
        rename = "enable_interpolation_in_file",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_interpolation_in_file: Option<Option<bool>>,
}

impl VariableAliasRequest {
    pub fn new(
        key: String,
        alias_scope: models::ApiVariableScopeEnum,
        alias_parent_id: uuid::Uuid,
    ) -> VariableAliasRequest {
        VariableAliasRequest {
            key,
            alias_scope,
            alias_parent_id,
            description: None,
            enable_interpolation_in_file: None,
        }
    }
}
