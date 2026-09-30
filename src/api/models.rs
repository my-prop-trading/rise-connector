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
    pub failed: Vec<FailedInvite>,
    #[serde(rename = "countAdded")]
    pub count_added: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FailedInvite {
    pub invite: String,
    pub error: String,
}
/// Talent (contractor) record from `GET v1/teams/{teamId}/talent`.
#[derive(Debug, Clone, Deserialize)]
pub struct Talent {
    pub email: String,
    pub nanoid: Option<String>,
    #[serde(rename = "riseId")]
    pub rise_id: Option<String>,
    pub role: Option<String>,
}

impl Talent {
    /// `riseId` only when it is an on-chain address; Rise may return a nanoid there instead.
    pub fn rise_id_address(&self) -> Option<&str> {
        self.rise_id
            .as_deref()
            .filter(|id| crate::utils::is_eth_address(id))
    }
}
