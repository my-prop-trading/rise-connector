use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    pub response: ResponseModel,
    pub result: Option<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseModel {
    pub status: String,
    pub code: i32,
    pub message: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SiweMessageResponse {
    pub message: String,
    pub wallet: String,
}

// Step 2: Sending the signature to get the token
#[derive(Debug, Clone, Serialize)]
pub struct SiweLoginRequest {
    pub wallet: String,
    pub message: String,
    pub signature: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SiweLoginResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateInviteRequest {
    #[serde(rename = "inviteList")]
    pub invite_list: Vec<String>, // emails or rise IDs
    pub anonymous: bool,
    pub company_riseid: String,
    pub role: Role, 
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Contractor,
    Client,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateInviteResponse {
    pub invited: Vec<String>,
    pub failed: Vec<String>,
    #[serde(rename = "countAdded")]
    pub count_added: i32,
}