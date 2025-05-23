/*
 * Qovery API
 *
 * - Qovery is the fastest way to deploy your full-stack apps on any Cloud provider. - ℹ️ The API is stable and still in development. 
 *
 * The version of the OpenAPI document: 1.0.3
 * Contact: support+api+documentation@qovery.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListServicesByEnvironmentId200ResponseResultsInner {
    Application(Box<models::Application>),
    ContainerResponse(Box<models::ContainerResponse>),
    Database(Box<models::Database>),
    HelmResponse(Box<models::HelmResponse>),
    JobResponse(Box<models::JobResponse>),
}

impl Default for ListServicesByEnvironmentId200ResponseResultsInner {
    fn default() -> Self {
        Self::Application(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JobType {
    #[serde(rename = "CRON")]
    Cron,
}

impl Default for JobType {
    fn default() -> JobType {
        Self::Cron
    }
}

