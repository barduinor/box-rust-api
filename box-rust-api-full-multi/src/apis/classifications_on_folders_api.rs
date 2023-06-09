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


/// struct for typed errors of method [`delete_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFoldersIdMetadataEnterpriseSecurityClassification6VmVochwUwoError {
    Status400(crate::models::ClientError),
    Status404(crate::models::ClientError),
    Status405(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFoldersIdMetadataEnterpriseSecurityClassification6VmVochwUwoError {
    Status403(crate::models::ClientError),
    Status404(crate::models::ClientError),
    Status405(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostFoldersIdMetadataEnterpriseSecurityClassification6VmVochwUwoError {
    Status400(crate::models::ClientError),
    Status404(crate::models::ClientError),
    Status409(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutFoldersIdMetadataEnterpriseSecurityClassification6VmVochwUwoError {
    Status400(crate::models::ClientError),
    Status500(crate::models::ClientError),
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}


/// Removes any classifications from a folder.  This API can also be called by including the enterprise ID in the URL explicitly, for example `/folders/:id//enterprise_12345/securityClassification-6VMVochwUWo`.
pub async fn delete_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo(configuration: &configuration::Configuration, folder_id: &str) -> Result<(), Error<DeleteFoldersIdMetadataEnterpriseSecurityClassification6VmVochwUwoError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/folders/{folder_id}/metadata/enterprise/securityClassification-6VMVochwUWo", local_var_configuration.base_path, folder_id=crate::apis::urlencode(folder_id));
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
        let local_var_entity: Option<DeleteFoldersIdMetadataEnterpriseSecurityClassification6VmVochwUwoError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves the classification metadata instance that has been applied to a folder.  This API can also be called by including the enterprise ID in the URL explicitly, for example `/folders/:id//enterprise_12345/securityClassification-6VMVochwUWo`.
pub async fn get_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo(configuration: &configuration::Configuration, folder_id: &str) -> Result<crate::models::Classification, Error<GetFoldersIdMetadataEnterpriseSecurityClassification6VmVochwUwoError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/folders/{folder_id}/metadata/enterprise/securityClassification-6VMVochwUWo", local_var_configuration.base_path, folder_id=crate::apis::urlencode(folder_id));
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
        let local_var_entity: Option<GetFoldersIdMetadataEnterpriseSecurityClassification6VmVochwUwoError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Adds a classification to a folder by specifying the label of the classification to add.  This API can also be called by including the enterprise ID in the URL explicitly, for example `/folders/:id//enterprise_12345/securityClassification-6VMVochwUWo`.
pub async fn post_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo(configuration: &configuration::Configuration, folder_id: &str, post_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo_request: Option<crate::models::PostFoldersIdMetadataEnterpriseSecurityClassification6VmVochwUwoRequest>) -> Result<crate::models::Classification, Error<PostFoldersIdMetadataEnterpriseSecurityClassification6VmVochwUwoError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/folders/{folder_id}/metadata/enterprise/securityClassification-6VMVochwUWo", local_var_configuration.base_path, folder_id=crate::apis::urlencode(folder_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&post_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostFoldersIdMetadataEnterpriseSecurityClassification6VmVochwUwoError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a classification on a folder.  The classification can only be updated if a classification has already been applied to the folder before. When editing classifications, only values are defined for the enterprise will be accepted.
pub async fn put_folders_id_metadata_enterprise_security_classification6_vm_vochw_uwo(configuration: &configuration::Configuration, folder_id: &str, put_folders_id_metadata_enterprise_security_classification_6_vm_vochw_uwo_request_inner: Option<Vec<crate::models::PutFoldersIdMetadataEnterpriseSecurityClassification6VmVochwUwoRequestInner>>) -> Result<crate::models::Classification, Error<PutFoldersIdMetadataEnterpriseSecurityClassification6VmVochwUwoError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/folders/{folder_id}/metadata/enterprise/securityClassification-6VMVochwUWo", local_var_configuration.base_path, folder_id=crate::apis::urlencode(folder_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&put_folders_id_metadata_enterprise_security_classification_6_vm_vochw_uwo_request_inner);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PutFoldersIdMetadataEnterpriseSecurityClassification6VmVochwUwoError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

