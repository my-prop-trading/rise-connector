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

// Step 1: Requesting the SIWE message
#[derive(Debug, Clone, Serialize)]
pub struct SiweMessageRequest {
    pub wallet: String,
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

#[derive(Debug, Clone, Serialize)]
pub struct CreateInviteRequest {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub company_riseid: Option<String>,
    pub light: bool, // If true, requires less KYC data (email, names only)
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateInviteResponse {
    pub id: String,
    pub status: String,
    pub email: String,
    // Add other fields as needed from the API response
}