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
pub struct ClusterRoutingTableResultsInner {
    #[serde(rename = "destination")]
    pub destination: String,
    #[serde(rename = "target")]
    pub target: String,
    #[serde(rename = "description")]
    pub description: String,
}

impl ClusterRoutingTableResultsInner {
    pub fn new(
        destination: String,
        target: String,
        description: String,
    ) -> ClusterRoutingTableResultsInner {
        ClusterRoutingTableResultsInner {
            destination,
            target,
            description,
        }
    }
}
