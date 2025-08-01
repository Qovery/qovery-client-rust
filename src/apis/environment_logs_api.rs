/*
 * Qovery API
 *
 * - Qovery is the fastest way to deploy your full-stack apps on any Cloud provider. - ℹ️ The API is stable and still in development.
 *
 * The version of the OpenAPI document: 1.0.4
 * Contact: support+api+documentation@qovery.com
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for typed errors of method [`list_environment_log`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListEnvironmentLogError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_environment_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListEnvironmentLogsError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// This returns the last 1000 environment deployment logs.
pub async fn list_environment_log(
    configuration: &configuration::Configuration,
    environment_id: &str,
) -> Result<models::EnvironmentLogResponseList, Error<ListEnvironmentLogError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_environment_id = environment_id;

    let uri_str = format!(
        "{}/environment/{environmentId}/log",
        configuration.base_path,
        environmentId = crate::apis::urlencode(p_environment_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::EnvironmentLogResponseList`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::EnvironmentLogResponseList`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListEnvironmentLogError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This returns the last 1000 environment deployment logs v2
pub async fn list_environment_logs(
    configuration: &configuration::Configuration,
    environment_id: &str,
    version: Option<&str>,
) -> Result<Vec<models::EnvironmentLogs>, Error<ListEnvironmentLogsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_environment_id = environment_id;
    let p_version = version;

    let uri_str = format!(
        "{}/environment/{environmentId}/logs",
        configuration.base_path,
        environmentId = crate::apis::urlencode(p_environment_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_version {
        req_builder = req_builder.query(&[("version", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::EnvironmentLogs&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::EnvironmentLogs&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListEnvironmentLogsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
