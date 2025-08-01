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

/// struct for typed errors of method [`organization_github_app_connect`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrganizationGithubAppConnectError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`organization_github_app_disconnect`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrganizationGithubAppDisconnectError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

pub async fn organization_github_app_connect(
    configuration: &configuration::Configuration,
    organization_id: &str,
    organization_github_app_connect_request: Option<models::OrganizationGithubAppConnectRequest>,
) -> Result<(), Error<OrganizationGithubAppConnectError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_organization_id = organization_id;
    let p_organization_github_app_connect_request = organization_github_app_connect_request;

    let uri_str = format!(
        "{}/organization/{organizationId}/github/connect",
        configuration.base_path,
        organizationId = crate::apis::urlencode(p_organization_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

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
    req_builder = req_builder.json(&p_organization_github_app_connect_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<OrganizationGithubAppConnectError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn organization_github_app_disconnect(
    configuration: &configuration::Configuration,
    organization_id: &str,
    force: Option<bool>,
) -> Result<(), Error<OrganizationGithubAppDisconnectError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_organization_id = organization_id;
    let p_force = force;

    let uri_str = format!(
        "{}/organization/{organizationId}/github/disconnect",
        configuration.base_path,
        organizationId = crate::apis::urlencode(p_organization_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref param_value) = p_force {
        req_builder = req_builder.query(&[("force", &param_value.to_string())]);
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

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<OrganizationGithubAppDisconnectError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
