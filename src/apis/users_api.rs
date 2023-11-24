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

/// struct for passing parameters to the method [`create_user`]
#[derive(Clone, Debug)]
pub struct CreateUserParams {
    /// The details of the user to create.
    pub create_user_request: Option<crate::models::CreateUserRequest>
}

/// struct for passing parameters to the method [`delete_user`]
#[derive(Clone, Debug)]
pub struct DeleteUserParams {
    /// The user's id.
    pub id: String,
    /// Delete all data and remove the user's profile from all of Kinde, including the subscriber list
    pub is_delete_profile: Option<bool>
}

/// struct for passing parameters to the method [`get_user_data`]
#[derive(Clone, Debug)]
pub struct GetUserDataParams {
    /// The user's id.
    pub id: String,
    /// Specify additional data to retrieve. Use \"organizations\" and/or \"identities\".
    pub expand: Option<String>
}

/// struct for passing parameters to the method [`get_users`]
#[derive(Clone, Debug)]
pub struct GetUsersParams {
    /// Field and order to sort the result by.
    pub sort: Option<String>,
    /// Number of results per page. Defaults to 10 if parameter not sent.
    pub page_size: Option<i32>,
    /// ID of the user to filter by.
    pub user_id: Option<String>,
    /// A string to get the next page of results if there are more results.
    pub next_token: Option<String>,
    /// Filter the results by email address. The query string should be comma separated and url encoded.
    pub email: Option<String>,
    /// Specify additional data to retrieve. Use \"organizations\" and/or \"identities\".
    pub expand: Option<String>
}

/// struct for passing parameters to the method [`refresh_user_claims`]
#[derive(Clone, Debug)]
pub struct RefreshUserClaimsParams {
    /// The id of the user whose claims needs to be updated.
    pub user_id: String
}

/// struct for passing parameters to the method [`update_user`]
#[derive(Clone, Debug)]
pub struct UpdateUserParams {
    /// The user to update.
    pub update_user_request: crate::models::UpdateUserRequest,
    /// The user's id.
    pub id: Option<String>
}

/// struct for passing parameters to the method [`update_user_feature_flag_override`]
#[derive(Clone, Debug)]
pub struct UpdateUserFeatureFlagOverrideParams {
    /// The identifier for the user
    pub user_id: String,
    /// The identifier for the feature flag
    pub feature_flag_key: String,
    /// Override value
    pub value: String
}


/// struct for typed errors of method [`create_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateUserError {
    Status400(crate::models::ErrorResponse),
    Status403(),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteUserError {
    Status400(crate::models::ErrorResponse),
    Status403(),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user_data`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserDataError {
    Status400(crate::models::ErrorResponse),
    Status403(),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsersError {
    Status403(),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`refresh_user_claims`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RefreshUserClaimsError {
    Status400(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateUserError {
    Status400(crate::models::ErrorResponse),
    Status403(),
    Status429(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_user_feature_flag_override`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateUserFeatureFlagOverrideError {
    Status400(crate::models::ErrorResponse),
    Status403(),
    Status429(),
    UnknownValue(serde_json::Value),
}


/// Creates a user record and optionally zero or more identities for the user. An example identity could be the email address of the user. 
pub async fn create_user(configuration: &configuration::Configuration, params: CreateUserParams) -> Result<crate::models::CreateUserResponse, Error<CreateUserError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let create_user_request = params.create_user_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/user", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&create_user_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a user record. 
pub async fn delete_user(configuration: &configuration::Configuration, params: DeleteUserParams) -> Result<crate::models::SuccessResponse, Error<DeleteUserError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let is_delete_profile = params.is_delete_profile;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/user", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("id", &id.to_string())]);
    if let Some(ref local_var_str) = is_delete_profile {
        local_var_req_builder = local_var_req_builder.query(&[("is_delete_profile", &local_var_str.to_string())]);
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
        let local_var_entity: Option<DeleteUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a user record. 
pub async fn get_user_data(configuration: &configuration::Configuration, params: GetUserDataParams) -> Result<crate::models::User, Error<GetUserDataError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let expand = params.expand;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/user", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("id", &id.to_string())]);
    if let Some(ref local_var_str) = expand {
        local_var_req_builder = local_var_req_builder.query(&[("expand", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetUserDataError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The returned list can be sorted by full name or email address in ascending or descending order. The number of records to return at a time can also be controlled using the `page_size` query string parameter. 
pub async fn get_users(configuration: &configuration::Configuration, params: GetUsersParams) -> Result<crate::models::UsersResponse, Error<GetUsersError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let sort = params.sort;
    let page_size = params.page_size;
    let user_id = params.user_id;
    let next_token = params.next_token;
    let email = params.email;
    let expand = params.expand;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/users", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_id {
        local_var_req_builder = local_var_req_builder.query(&[("user_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = next_token {
        local_var_req_builder = local_var_req_builder.query(&[("next_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = email {
        local_var_req_builder = local_var_req_builder.query(&[("email", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = expand {
        local_var_req_builder = local_var_req_builder.query(&[("expand", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetUsersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Refreshes the user's claims and invalidates the current cache. 
pub async fn refresh_user_claims(configuration: &configuration::Configuration, params: RefreshUserClaimsParams) -> Result<crate::models::SuccessResponse, Error<RefreshUserClaimsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let user_id = params.user_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/users/{user_id}/refresh_claims", local_var_configuration.base_path, user_id=crate::apis::urlencode(user_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
        let local_var_entity: Option<RefreshUserClaimsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a user record. 
pub async fn update_user(configuration: &configuration::Configuration, params: UpdateUserParams) -> Result<crate::models::UpdateUserResponse, Error<UpdateUserError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let update_user_request = params.update_user_request;
    let id = params.id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/user", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = id {
        local_var_req_builder = local_var_req_builder.query(&[("id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&update_user_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update user feature flag override.
pub async fn update_user_feature_flag_override(configuration: &configuration::Configuration, params: UpdateUserFeatureFlagOverrideParams) -> Result<crate::models::SuccessResponse, Error<UpdateUserFeatureFlagOverrideError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let user_id = params.user_id;
    let feature_flag_key = params.feature_flag_key;
    let value = params.value;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/users/{user_id}/feature_flags/{feature_flag_key}", local_var_configuration.base_path, user_id=crate::apis::urlencode(user_id), feature_flag_key=crate::apis::urlencode(feature_flag_key));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("value", &value.to_string())]);
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
        let local_var_entity: Option<UpdateUserFeatureFlagOverrideError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

