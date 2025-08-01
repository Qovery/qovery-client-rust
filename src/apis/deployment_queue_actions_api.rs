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

/// struct for typed errors of method [`cancel_deployment_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelDeploymentRequestError {
    Status401(),
    Status403(),
    Status404(),
    Status409(),
    UnknownValue(serde_json::Value),
}

/// Cancel the a deployment request.
pub async fn cancel_deployment_request(
    configuration: &configuration::Configuration,
    deployment_request_id: &str,
    body: Option<serde_json::Value>,
) -> Result<(), Error<CancelDeploymentRequestError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_deployment_request_id = deployment_request_id;
    let p_body = body;

    let uri_str = format!(
        "{}/deploymentQueue/{deploymentRequestId}/cancelDeployment",
        configuration.base_path,
        deploymentRequestId = crate::apis::urlencode(p_deployment_request_id)
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
    req_builder = req_builder.json(&p_body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<CancelDeploymentRequestError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
