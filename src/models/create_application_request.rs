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
pub struct CreateApplicationRequest {
    /// The application's name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The application's type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl CreateApplicationRequest {
    pub fn new() -> CreateApplicationRequest {
        CreateApplicationRequest {
            name: None,
            r#type: None,
        }
    }
}

/// The application's type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "reg")]
    Reg,
    #[serde(rename = "spa")]
    Spa,
    #[serde(rename = "m2m")]
    M2m,
}

impl Default for Type {
    fn default() -> Type {
        Self::Reg
    }
}

