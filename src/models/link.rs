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
pub struct Link {
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The port from which the service is reachable from within the cluster
    #[serde(rename = "internal_port", skip_serializing_if = "Option::is_none")]
    pub internal_port: Option<i32>,
    /// The port from which the service is reachable from externally (i.e: 443 for HTTPS)
    #[serde(rename = "external_port", skip_serializing_if = "Option::is_none")]
    pub external_port: Option<i32>,
    /// True if the domain is managed by Qovery, false if it belongs to the user
    #[serde(rename = "is_qovery_domain", skip_serializing_if = "Option::is_none")]
    pub is_qovery_domain: Option<bool>,
    /// Indicate if the link is using the root of the domain and not one derivated from port i.e: p8080.zxxxx.jvm.worl      => is_default = false, is_qovery = true zxxxx.jvm.world           => is_default = true, is_qovery = true p8080-my-super-domain.com => is_default = false, is_qovery = false my-super-domain.com       => is_default = true, is_qovery = false
    #[serde(rename = "is_default", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

impl Link {
    pub fn new() -> Link {
        Link {
            url: None,
            internal_port: None,
            external_port: None,
            is_qovery_domain: None,
            is_default: None,
        }
    }
}
