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

/// HelmResponseAllOfValuesOverride : Specify helm values you want to set or override
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HelmResponseAllOfValuesOverride {
    /// The input is in json array format: [ [$KEY,$VALUE], [...] ]
    #[serde(rename = "set", skip_serializing_if = "Option::is_none")]
    pub set: Option<Vec<Vec<String>>>,
    /// The input is in json array format: [ [$KEY,$VALUE], [...] ]
    #[serde(rename = "set_string", skip_serializing_if = "Option::is_none")]
    pub set_string: Option<Vec<Vec<String>>>,
    /// The input is in json array format: [ [$KEY,$VALUE], [...] ]
    #[serde(rename = "set_json", skip_serializing_if = "Option::is_none")]
    pub set_json: Option<Vec<Vec<String>>>,
    #[serde(
        rename = "file",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub file: Option<Option<models::HelmResponseAllOfValuesOverrideFile>>,
}

impl HelmResponseAllOfValuesOverride {
    /// Specify helm values you want to set or override
    pub fn new() -> HelmResponseAllOfValuesOverride {
        HelmResponseAllOfValuesOverride {
            set: None,
            set_string: None,
            set_json: None,
            file: None,
        }
    }
}
