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
pub struct VariableAlias {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(
        rename = "value",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<Option<String>>,
    #[serde(rename = "mount_path")]
    pub mount_path: String,
    #[serde(rename = "scope")]
    pub scope: models::ApiVariableScopeEnum,
    #[serde(rename = "variable_type")]
    pub variable_type: models::ApiVariableTypeEnum,
}

impl VariableAlias {
    pub fn new(
        id: uuid::Uuid,
        key: String,
        mount_path: String,
        scope: models::ApiVariableScopeEnum,
        variable_type: models::ApiVariableTypeEnum,
    ) -> VariableAlias {
        VariableAlias {
            id,
            key,
            value: None,
            mount_path,
            scope,
            variable_type,
        }
    }
}
