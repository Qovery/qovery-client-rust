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
pub struct Commit {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "git_commit_id")]
    pub git_commit_id: String,
    #[serde(rename = "tag")]
    pub tag: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "author_name")]
    pub author_name: String,
    #[serde(rename = "author_avatar_url", skip_serializing_if = "Option::is_none")]
    pub author_avatar_url: Option<String>,
    #[serde(rename = "commit_page_url", skip_serializing_if = "Option::is_none")]
    pub commit_page_url: Option<String>,
}

impl Commit {
    pub fn new(
        created_at: String,
        git_commit_id: String,
        tag: String,
        message: String,
        author_name: String,
    ) -> Commit {
        Commit {
            created_at,
            git_commit_id,
            tag,
            message,
            author_name,
            author_avatar_url: None,
            commit_page_url: None,
        }
    }
}
