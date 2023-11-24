/*
 * Kinde Management API
 *
 * Provides endpoints to manage your Kinde Businesses
 *
 * The version of the OpenAPI document: 1
 * Contact: support@kinde.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateOrganizationUsersRequestUsersInner {
    /// The users id.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Optional operation, set to 'delete' to remove the user from the organization.
    #[serde(rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    /// Role keys to assign to the user.
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    /// Permission keys to assign to the user.
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

impl UpdateOrganizationUsersRequestUsersInner {
    pub fn new() -> UpdateOrganizationUsersRequestUsersInner {
        UpdateOrganizationUsersRequestUsersInner {
            id: None,
            operation: None,
            roles: None,
            permissions: None,
        }
    }
}


