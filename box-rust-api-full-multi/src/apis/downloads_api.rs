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


/// struct for typed errors of method [`get_files_id_content`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFilesIdContentError {
    DefaultResponse(crate::models::ClientError),
    UnknownValue(serde_json::Value),
}


/// Returns the contents of a file in binary format.
pub async fn get_files_id_content(configuration: &configuration::Configuration, file_id: &str, range: Option<&str>, boxapi: Option<&str>, version: Option<&str>, access_token: Option<&str>) -> Result<(), Error<GetFilesIdContentError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/files/{file_id}/content", local_var_configuration.base_path, file_id=crate::apis::urlencode(file_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = version {
        local_var_req_builder = local_var_req_builder.query(&[("version", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = access_token {
        local_var_req_builder = local_var_req_builder.query(&[("access_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = range {
        local_var_req_builder = local_var_req_builder.header("range", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = boxapi {
        local_var_req_builder = local_var_req_builder.header("boxapi", local_var_param_value.to_string());
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
        let local_var_entity: Option<GetFilesIdContentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

