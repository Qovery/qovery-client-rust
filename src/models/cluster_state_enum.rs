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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ClusterStateEnum {
    #[serde(rename = "BUILDING")]
    Building,
    #[serde(rename = "BUILD_ERROR")]
    BuildError,
    #[serde(rename = "CANCELED")]
    Canceled,
    #[serde(rename = "CANCELING")]
    Canceling,
    #[serde(rename = "DELETED")]
    Deleted,
    #[serde(rename = "DELETE_ERROR")]
    DeleteError,
    #[serde(rename = "DELETE_QUEUED")]
    DeleteQueued,
    #[serde(rename = "DELETING")]
    Deleting,
    #[serde(rename = "DEPLOYED")]
    Deployed,
    #[serde(rename = "DEPLOYING")]
    Deploying,
    #[serde(rename = "DEPLOYMENT_ERROR")]
    DeploymentError,
    #[serde(rename = "DEPLOYMENT_QUEUED")]
    DeploymentQueued,
    #[serde(rename = "DRY_RUN")]
    DryRun,
    #[serde(rename = "QUEUED")]
    Queued,
    #[serde(rename = "READY")]
    Ready,
    #[serde(rename = "STOPPED")]
    Stopped,
    #[serde(rename = "STOPPING")]
    Stopping,
    #[serde(rename = "STOP_ERROR")]
    StopError,
    #[serde(rename = "STOP_QUEUED")]
    StopQueued,
    #[serde(rename = "RESTART_QUEUED")]
    RestartQueued,
    #[serde(rename = "RESTARTING")]
    Restarting,
    #[serde(rename = "RESTARTED")]
    Restarted,
    #[serde(rename = "RESTART_ERROR")]
    RestartError,
    #[serde(rename = "INVALID_CREDENTIALS")]
    InvalidCredentials,
}

impl std::fmt::Display for ClusterStateEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Building => write!(f, "BUILDING"),
            Self::BuildError => write!(f, "BUILD_ERROR"),
            Self::Canceled => write!(f, "CANCELED"),
            Self::Canceling => write!(f, "CANCELING"),
            Self::Deleted => write!(f, "DELETED"),
            Self::DeleteError => write!(f, "DELETE_ERROR"),
            Self::DeleteQueued => write!(f, "DELETE_QUEUED"),
            Self::Deleting => write!(f, "DELETING"),
            Self::Deployed => write!(f, "DEPLOYED"),
            Self::Deploying => write!(f, "DEPLOYING"),
            Self::DeploymentError => write!(f, "DEPLOYMENT_ERROR"),
            Self::DeploymentQueued => write!(f, "DEPLOYMENT_QUEUED"),
            Self::DryRun => write!(f, "DRY_RUN"),
            Self::Queued => write!(f, "QUEUED"),
            Self::Ready => write!(f, "READY"),
            Self::Stopped => write!(f, "STOPPED"),
            Self::Stopping => write!(f, "STOPPING"),
            Self::StopError => write!(f, "STOP_ERROR"),
            Self::StopQueued => write!(f, "STOP_QUEUED"),
            Self::RestartQueued => write!(f, "RESTART_QUEUED"),
            Self::Restarting => write!(f, "RESTARTING"),
            Self::Restarted => write!(f, "RESTARTED"),
            Self::RestartError => write!(f, "RESTART_ERROR"),
            Self::InvalidCredentials => write!(f, "INVALID_CREDENTIALS"),
        }
    }
}

impl Default for ClusterStateEnum {
    fn default() -> ClusterStateEnum {
        Self::Building
    }
}
