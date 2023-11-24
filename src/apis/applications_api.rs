/*
 * Kinde Management API
 *
 * Provides endpoints to manage your Kinde Businesses
 *
 * The version of the OpenAPI document: 1
 * Contact: support@kinde.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_application`]
#[derive(Clone, Debug)]
pub struct CreateApplicationParams {
    /// Application details.
    pub create_application_request: Option<crate::models::CreateApplicationRequest>
}

/// struct for passing parameters to the method [`delete_application`]
#[derive(Clone, Debug)]
pub struct DeleteApplicationParams {
    /// The identifier for the application.
    pub application_id: String
}

/// struct for passing parameters to the method [`get_application`]
#[derive(Clone, Debug)]
pub struct GetApplicationParams {
    /// The identifier for the application.
    pub application_id: String
}

/// struct for passing parameters to the method [`get_applications`]
#[derive(Clone, Debug)]
pub struct GetApplicationsParams {
    /// Field and order to sort the result by.
    pub sort: Option<String>,
    /// Number of results per page. Defaults to 10 if parameter not sent.
    pub page_size: Option<i32>,
    /// A string to get the next page of results if there are more results.
    pub next_token: Option<String>
}

/// struct for passing parameters to the method [`update_application`]
#[derive(Clone, Debug)]
pub struct UpdateApplicationParams {
    /// The identifier for the application.
    pub application_id: String,
    /// Application details.
    pub update_application_request: Option<crate::models::UpdateApplicationRequest>
}


/// struct for typed errors of method [`create_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateApplicationError {
    Status400(crate::models::ErrorResponse),
    Status403(),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApplicationError {
    Status400(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationError {
    Status400(crate::models::ErrorResponse),
    Status403(),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_applications`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationsError {
    Status403(),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApplicationError {
    Status403(),
    Status400(crate::models::ErrorResponse),
    Status429(),
    UnknownValue(serde_json::Value),
}


/// Create an application.
pub async fn create_application(configuration: &configuration::Configuration, params: CreateApplicationParams) -> Result<crate::models::CreateApplicationResponse, Error<CreateApplicationError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let create_application_request = params.create_application_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/applications", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&create_application_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateApplicationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete application. 
pub async fn delete_application(configuration: &configuration::Configuration, params: DeleteApplicationParams) -> Result<crate::models::SuccessResponse, Error<DeleteApplicationError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let application_id = params.application_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/applications/{application_id}", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteApplicationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets an application given the application's id. 
pub async fn get_application(configuration: &configuration::Configuration, params: GetApplicationParams) -> Result<crate::models::GetApplicationResponse, Error<GetApplicationError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let application_id = params.application_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/applications/{application_id}", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetApplicationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a list of applications. 
pub async fn get_applications(configuration: &configuration::Configuration, params: GetApplicationsParams) -> Result<crate::models::GetApplicationsResponse, Error<GetApplicationsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let sort = params.sort;
    let page_size = params.page_size;
    let next_token = params.next_token;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/applications", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = next_token {
        local_var_req_builder = local_var_req_builder.query(&[("next_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetApplicationsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update an application.
pub async fn update_application(configuration: &configuration::Configuration, params: UpdateApplicationParams) -> Result<(), Error<UpdateApplicationError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let application_id = params.application_id;
    let update_application_request = params.update_application_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/applications/{application_id}", local_var_configuration.base_path, application_id=crate::apis::urlencode(application_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&update_application_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<UpdateApplicationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

