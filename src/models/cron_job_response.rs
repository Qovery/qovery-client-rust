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
pub struct CronJobResponse {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "environment")]
    pub environment: models::ReferenceObject,
    /// Maximum cpu that can be allocated to the job based on organization cluster configuration. unit is millicores (m). 1000m = 1 cpu
    #[serde(rename = "maximum_cpu")]
    pub maximum_cpu: i32,
    /// Maximum memory that can be allocated to the job based on organization cluster configuration. unit is MB. 1024 MB = 1GB
    #[serde(rename = "maximum_memory")]
    pub maximum_memory: i32,
    /// name is case insensitive
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// unit is millicores (m). 1000m = 1 cpu
    #[serde(rename = "cpu")]
    pub cpu: i32,
    /// unit is MB. 1024 MB = 1GB
    #[serde(rename = "memory")]
    pub memory: i32,
    /// Maximum number of restart allowed before the job is considered as failed 0 means that no restart/crash of the job is allowed
    #[serde(rename = "max_nb_restart", skip_serializing_if = "Option::is_none")]
    pub max_nb_restart: Option<i32>,
    /// Maximum number of seconds allowed for the job to run before killing it and mark it as failed
    #[serde(
        rename = "max_duration_seconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_duration_seconds: Option<i32>,
    /// Indicates if the 'environment preview option' is enabled for this container.   If enabled, a preview environment will be automatically cloned when `/preview` endpoint is called.   If not specified, it takes the value of the `auto_preview` property from the associated environment.
    #[serde(rename = "auto_preview")]
    pub auto_preview: bool,
    /// Port where to run readiness and liveliness probes checks. The port will not be exposed externally
    #[serde(
        rename = "port",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub port: Option<Option<i32>>,
    #[serde(rename = "source")]
    pub source: models::BaseJobResponseAllOfSource,
    #[serde(rename = "healthchecks")]
    pub healthchecks: models::Healthcheck,
    /// Specify if the job will be automatically updated after receiving a new image tag or a new commit according to the source type.  The new image tag shall be communicated via the \"Auto Deploy job\" endpoint https://api-doc.qovery.com/#tag/Jobs/operation/autoDeployJobEnvironments
    #[serde(rename = "auto_deploy", skip_serializing_if = "Option::is_none")]
    pub auto_deploy: Option<bool>,
    /// Icon URI representing the job.
    #[serde(rename = "icon_uri")]
    pub icon_uri: String,
    #[serde(
        rename = "service_type",
        default = "models::service_type_enum::service_type_job"
    )]
    pub service_type: models::ServiceTypeEnum,
    #[serde(rename = "job_type", default = "models::job_type_enum::job_type_cron")]
    pub job_type: models::JobTypeEnum,
    #[serde(rename = "schedule")]
    pub schedule: models::CronJobResponseAllOfSchedule,
    #[serde(rename = "annotations_groups", skip_serializing_if = "Option::is_none")]
    pub annotations_groups: Option<Vec<models::OrganizationAnnotationsGroupResponse>>,
    #[serde(rename = "labels_groups", skip_serializing_if = "Option::is_none")]
    pub labels_groups: Option<Vec<models::OrganizationLabelsGroupResponse>>,
}

impl CronJobResponse {
    pub fn new(
        id: uuid::Uuid,
        created_at: String,
        environment: models::ReferenceObject,
        maximum_cpu: i32,
        maximum_memory: i32,
        name: String,
        cpu: i32,
        memory: i32,
        auto_preview: bool,
        source: models::BaseJobResponseAllOfSource,
        healthchecks: models::Healthcheck,
        icon_uri: String,
        service_type: models::ServiceTypeEnum,
        job_type: models::JobTypeEnum,
        schedule: models::CronJobResponseAllOfSchedule,
    ) -> CronJobResponse {
        CronJobResponse {
            id,
            created_at,
            updated_at: None,
            environment,
            maximum_cpu,
            maximum_memory,
            name,
            description: None,
            cpu,
            memory,
            max_nb_restart: None,
            max_duration_seconds: None,
            auto_preview,
            port: None,
            source,
            healthchecks,
            auto_deploy: None,
            icon_uri,
            service_type,
            job_type,
            schedule,
            annotations_groups: None,
            labels_groups: None,
        }
    }
}
