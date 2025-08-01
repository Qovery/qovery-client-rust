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
pub struct DeleteMemberRequest {
    #[serde(rename = "user_id")]
    pub user_id: uuid::Uuid,
}

impl DeleteMemberRequest {
    pub fn new(user_id: uuid::Uuid) -> DeleteMemberRequest {
        DeleteMemberRequest { user_id }
    }
}
