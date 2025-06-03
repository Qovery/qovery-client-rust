use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String),
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            return Self::Json;
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return Self::Unsupported(content_type.to_string());
        }
    }
}

pub mod account_info_api;
pub mod application_actions_api;
pub mod application_configuration_api;
pub mod application_custom_domain_api;
pub mod application_deployment_history_api;
pub mod application_deployment_restriction_api;
pub mod application_environment_variable_api;
pub mod application_logs_api;
pub mod application_main_calls_api;
pub mod application_secret_api;
pub mod applications_api;
pub mod backups_api;
pub mod billing_api;
pub mod cloud_provider_api;
pub mod cloud_provider_credentials_api;
pub mod clusters_api;
pub mod container_actions_api;
pub mod container_configuration_api;
pub mod container_custom_domain_api;
pub mod container_deployment_history_api;
pub mod container_environment_variable_api;
pub mod container_logs_api;
pub mod container_main_calls_api;
pub mod container_registries_api;
pub mod container_secret_api;
pub mod containers_api;
pub mod database_actions_api;
pub mod database_application_api;
pub mod database_deployment_history_api;
pub mod database_main_calls_api;
pub mod databases_api;
pub mod default_api;
pub mod deployment_queue_actions_api;
pub mod deployment_stage_main_calls_api;
pub mod environment_actions_api;
pub mod environment_api;
pub mod environment_deployment_history_api;
pub mod environment_deployment_rule_api;
pub mod environment_export_api;
pub mod environment_logs_api;
pub mod environment_main_calls_api;
pub mod environment_secret_api;
pub mod environment_variable_api;
pub mod environments_api;
pub mod git_repositories_api;
pub mod github_app_api;
pub mod helm_actions_api;
pub mod helm_configuration_api;
pub mod helm_custom_domain_api;
pub mod helm_deployment_history_api;
pub mod helm_deployment_restriction_api;
pub mod helm_main_calls_api;
pub mod helm_repositories_api;
pub mod helms_api;
pub mod job_actions_api;
pub mod job_configuration_api;
pub mod job_deployment_history_api;
pub mod job_deployment_restriction_api;
pub mod job_environment_variable_api;
pub mod job_main_calls_api;
pub mod job_secret_api;
pub mod jobs_api;
pub mod lifecycle_template_main_calls_api;
pub mod members_api;
pub mod organization_account_git_repositories_api;
pub mod organization_annotations_group_api;
pub mod organization_api_token_api;
pub mod organization_cluster_lock_api;
pub mod organization_custom_role_api;
pub mod organization_event_api;
pub mod organization_labels_group_api;
pub mod organization_main_calls_api;
pub mod organization_webhook_api;
pub mod project_deployment_rule_api;
pub mod project_environment_variable_api;
pub mod project_main_calls_api;
pub mod project_secret_api;
pub mod projects_api;
pub mod referral_rewards_api;
pub mod service_status_api;
pub mod terraform_actions_api;
pub mod terraform_configuration_api;
pub mod terraform_deployment_history_api;
pub mod terraform_deployment_restriction_api;
pub mod terraform_main_calls_api;
pub mod terraforms_api;
pub mod user_sign_up_api;
pub mod variable_main_calls_api;

pub mod configuration;
