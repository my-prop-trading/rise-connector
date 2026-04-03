use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookPayload {
    #[serde(flatten)]
    pub header: WebhookHeader,
    pub entity: WebhookEntity,
    pub meta: Option<WebhookMeta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookEntity {
    pub id: String,
    pub email: String,
    pub status: String,
    #[serde(rename = "rise_id")]
    pub rise_id: Option<String>, 
    pub role: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookHeader {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookMeta {
    pub source: String,
    pub version: String,
}