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


/// struct for typed errors of method [`delete_collaborations_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCollaborationsIdError {
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_collaborations_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCollaborationsIdError {
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_collaborations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostCollaborationsError {
    Status403(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_collaborations_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutCollaborationsIdError {
    Status403(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}


/// Deletes a single collaboration.
pub async fn delete_collaborations_id(configuration: &configuration::Configuration, collaboration_id: &str) -> Result<(), Error<DeleteCollaborationsIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/collaborations/{collaboration_id}", local_var_configuration.base_path, collaboration_id=crate::apis::urlencode(collaboration_id));
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
        let local_var_entity: Option<DeleteCollaborationsIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves a single collaboration.
pub async fn get_collaborations_id(configuration: &configuration::Configuration, collaboration_id: &str, fields: Option<Vec<String>>) -> Result<crate::models::Collaboration, Error<GetCollaborationsIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/collaborations/{collaboration_id}", local_var_configuration.base_path, collaboration_id=crate::apis::urlencode(collaboration_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<GetCollaborationsIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Adds a collaboration for a single user or a single group to a file or folder.  Collaborations can be created using email address, user IDs, or a group IDs.  If a collaboration is being created with a group, access to this endpoint is dependent on the group's ability to be invited.  If collaboration is in `pending` status, the following fields are redacted: - `login` and `name` are hidden if a collaboration was created using `user_id`, -  `name` is hidden if a collaboration was created using `login`.
pub async fn post_collaborations(configuration: &configuration::Configuration, fields: Option<Vec<String>>, notify: Option<bool>, post_collaborations_request: Option<crate::models::PostCollaborationsRequest>) -> Result<crate::models::Collaboration, Error<PostCollaborationsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/collaborations", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = notify {
        local_var_req_builder = local_var_req_builder.query(&[("notify", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&post_collaborations_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostCollaborationsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a collaboration. Can be used to change the owner of an item, or to accept collaboration invites.
pub async fn put_collaborations_id(configuration: &configuration::Configuration, collaboration_id: &str, put_collaborations_id_request: Option<crate::models::PutCollaborationsIdRequest>) -> Result<crate::models::Collaboration, Error<PutCollaborationsIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/collaborations/{collaboration_id}", local_var_configuration.base_path, collaboration_id=crate::apis::urlencode(collaboration_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&put_collaborations_id_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PutCollaborationsIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

