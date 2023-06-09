/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`get_file_version_legal_holds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFileVersionLegalHoldsError {
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_file_version_legal_holds_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFileVersionLegalHoldsIdError {
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}


/// Get a list of file versions on legal hold for a legal hold assignment.  Due to ongoing re-architecture efforts this API might not return all file versions for this policy ID.  Instead, this API will only return file versions held in the legacy architecture. Two new endpoints will available to request any file versions held in the new architecture.  For file versions held in the new architecture, the `GET /legal_hold_policy_assignments/:id/file_versions_on_hold` API can be used to return all past file versions available for this policy assignment, and the `GET /legal_hold_policy_assignments/:id/files_on_hold` API can be used to return any current (latest) versions of a file under legal hold.  The `GET /legal_hold_policy_assignments?policy_id={id}` API can be used to find a list of policy assignments for a given policy ID.  Once the re-architecture is completed this API will be deprecated.
pub async fn get_file_version_legal_holds(configuration: &configuration::Configuration, policy_id: &str, marker: Option<&str>, limit: Option<i64>) -> Result<crate::models::FileVersionLegalHolds, Error<GetFileVersionLegalHoldsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/file_version_legal_holds", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("policy_id", &policy_id.to_string())]);
    if let Some(ref local_var_str) = marker {
        local_var_req_builder = local_var_req_builder.query(&[("marker", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetFileVersionLegalHoldsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves information about the legal hold policies assigned to a file version.
pub async fn get_file_version_legal_holds_id(configuration: &configuration::Configuration, file_version_legal_hold_id: &str) -> Result<crate::models::FileVersionLegalHold, Error<GetFileVersionLegalHoldsIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/file_version_legal_holds/{file_version_legal_hold_id}", local_var_configuration.base_path, file_version_legal_hold_id=crate::apis::urlencode(file_version_legal_hold_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetFileVersionLegalHoldsIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
