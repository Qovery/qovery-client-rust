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
pub struct JobRequestAllOfSourceDocker {
    #[serde(rename = "git_repository", skip_serializing_if = "Option::is_none")]
    pub git_repository: Option<models::ApplicationGitRepositoryRequest>,
    /// The path of the associated Dockerfile. Only if you are using build_mode = DOCKER
    #[serde(
        rename = "dockerfile_path",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub dockerfile_path: Option<Option<String>>,
    /// The content of your dockerfile if it is not stored inside your git repository
    #[serde(
        rename = "dockerfile_raw",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub dockerfile_raw: Option<Option<String>>,
    /// The target build stage in the Dockerfile to build
    #[serde(
        rename = "docker_target_build_stage",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub docker_target_build_stage: Option<Option<String>>,
}

impl JobRequestAllOfSourceDocker {
    pub fn new() -> JobRequestAllOfSourceDocker {
        JobRequestAllOfSourceDocker {
            git_repository: None,
            dockerfile_path: None,
            dockerfile_raw: None,
            docker_target_build_stage: None,
        }
    }
}
