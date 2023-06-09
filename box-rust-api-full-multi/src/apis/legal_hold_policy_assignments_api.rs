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


/// struct for typed errors of method [`delete_legal_hold_policy_assignments_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLegalHoldPolicyAssignmentsIdError {
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_legal_hold_policy_assignments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLegalHoldPolicyAssignmentsError {
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_legal_hold_policy_assignments_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLegalHoldPolicyAssignmentsIdError {
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_legal_hold_policy_assignments_id_file_versions_on_hold`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLegalHoldPolicyAssignmentsIdFileVersionsOnHoldError {
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_legal_hold_policy_assignments_id_files_on_hold`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLegalHoldPolicyAssignmentsIdFilesOnHoldError {
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_legal_hold_policy_assignments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostLegalHoldPolicyAssignmentsError {
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}


/// Remove a legal hold from an item.  This is an asynchronous process. The policy will not be fully removed yet when the response returns.
pub async fn delete_legal_hold_policy_assignments_id(configuration: &configuration::Configuration, legal_hold_policy_assignment_id: &str) -> Result<(), Error<DeleteLegalHoldPolicyAssignmentsIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/legal_hold_policy_assignments/{legal_hold_policy_assignment_id}", local_var_configuration.base_path, legal_hold_policy_assignment_id=crate::apis::urlencode(legal_hold_policy_assignment_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        Ok(())
    } else {
        let local_var_entity: Option<DeleteLegalHoldPolicyAssignmentsIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves a list of items a legal hold policy has been assigned to.
pub async fn get_legal_hold_policy_assignments(configuration: &configuration::Configuration, policy_id: &str, assign_to_type: Option<&str>, assign_to_id: Option<&str>, marker: Option<&str>, limit: Option<i64>, fields: Option<Vec<String>>) -> Result<crate::models::LegalHoldPolicyAssignments, Error<GetLegalHoldPolicyAssignmentsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/legal_hold_policy_assignments", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("policy_id", &policy_id.to_string())]);
    if let Some(ref local_var_str) = assign_to_type {
        local_var_req_builder = local_var_req_builder.query(&[("assign_to_type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = assign_to_id {
        local_var_req_builder = local_var_req_builder.query(&[("assign_to_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = marker {
        local_var_req_builder = local_var_req_builder.query(&[("marker", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
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
        let local_var_entity: Option<GetLegalHoldPolicyAssignmentsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a legal hold policy assignment.
pub async fn get_legal_hold_policy_assignments_id(configuration: &configuration::Configuration, legal_hold_policy_assignment_id: &str) -> Result<crate::models::LegalHoldPolicyAssignment, Error<GetLegalHoldPolicyAssignmentsIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/legal_hold_policy_assignments/{legal_hold_policy_assignment_id}", local_var_configuration.base_path, legal_hold_policy_assignment_id=crate::apis::urlencode(legal_hold_policy_assignment_id));
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
        let local_var_entity: Option<GetLegalHoldPolicyAssignmentsIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a list of previous file versions for a legal hold assignment.  In some cases you may only need the latest file versions instead. In these cases, use the `GET  /legal_hold_policy_assignments/:id/files_on_hold` API instead to return any current (latest) versions of a file for this legal hold policy assignment.  Due to ongoing re-architecture efforts this API might not return all files held for this policy ID. Instead, this API will only return past file versions held in the newly developed architecture. The `GET /file_version_legal_holds` API can be used to fetch current and past versions of files held within the legacy architecture.  The `GET /legal_hold_policy_assignments?policy_id={id}` API can be used to find a list of policy assignments for a given policy ID.
pub async fn get_legal_hold_policy_assignments_id_file_versions_on_hold(configuration: &configuration::Configuration, legal_hold_policy_assignment_id: &str, marker: Option<&str>, limit: Option<i64>, fields: Option<Vec<String>>) -> Result<crate::models::FileVersionLegalHolds, Error<GetLegalHoldPolicyAssignmentsIdFileVersionsOnHoldError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/legal_hold_policy_assignments/{legal_hold_policy_assignment_id}/file_versions_on_hold", local_var_configuration.base_path, legal_hold_policy_assignment_id=crate::apis::urlencode(legal_hold_policy_assignment_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = marker {
        local_var_req_builder = local_var_req_builder.query(&[("marker", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
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
        let local_var_entity: Option<GetLegalHoldPolicyAssignmentsIdFileVersionsOnHoldError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a list of current file versions for a legal hold assignment.  In some cases you may want to get previous file versions instead. In these cases, use the `GET  /legal_hold_policy_assignments/:id/file_versions_on_hold` API instead to return any previous versions of a file for this legal hold policy assignment.  Due to ongoing re-architecture efforts this API might not return all file versions held for this policy ID. Instead, this API will only return the latest file version held in the newly developed architecture. The `GET /file_version_legal_holds` API can be used to fetch current and past versions of files held within the legacy architecture.  The `GET /legal_hold_policy_assignments?policy_id={id}` API can be used to find a list of policy assignments for a given policy ID.
pub async fn get_legal_hold_policy_assignments_id_files_on_hold(configuration: &configuration::Configuration, legal_hold_policy_assignment_id: &str, marker: Option<&str>, limit: Option<i64>, fields: Option<Vec<String>>) -> Result<crate::models::FileVersionLegalHolds, Error<GetLegalHoldPolicyAssignmentsIdFilesOnHoldError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/legal_hold_policy_assignments/{legal_hold_policy_assignment_id}/files_on_hold", local_var_configuration.base_path, legal_hold_policy_assignment_id=crate::apis::urlencode(legal_hold_policy_assignment_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = marker {
        local_var_req_builder = local_var_req_builder.query(&[("marker", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
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
        let local_var_entity: Option<GetLegalHoldPolicyAssignmentsIdFilesOnHoldError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Assign a legal hold to a file, file version, folder, or user.
pub async fn post_legal_hold_policy_assignments(configuration: &configuration::Configuration, post_legal_hold_policy_assignments_request: Option<crate::models::PostLegalHoldPolicyAssignmentsRequest>) -> Result<crate::models::LegalHoldPolicyAssignment, Error<PostLegalHoldPolicyAssignmentsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/legal_hold_policy_assignments", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&post_legal_hold_policy_assignments_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostLegalHoldPolicyAssignmentsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

