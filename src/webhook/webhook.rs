use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookPayload {
    pub company_id: i64,
    #[serde(rename = "type")]
    pub event_type: String,
    pub timestamp: i64,
    pub invite: InviteDetails,
    pub idempotent_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteDetails {
    pub email: String,
    pub role: String,
    pub uuid: String,
    #[serde(rename = "riseId")]
    pub rise_id: Option<String>,
    pub converted: i32,
    pub company: i64,
}
