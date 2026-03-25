
use rise_connector::api::{api_client::{RestApiClient, RestApiConfig}, models::CreateInviteRequest};
use std::time::Duration;
use tokio::time::Instant;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let config = ExampleApiConfig;
    let rise_api = RestApiClient::new(config);
    let instant = Instant::now();

    authorize(&rise_api).await;

    // 2. Create Invite
    create_invitation(&rise_api).await;

    println!("elapsed time: {:?}", instant.elapsed());
}

pub fn create_invite_req() -> CreateInviteRequest {
    let unique_email = format!("test-{}@mailinator.com", Uuid::new_v4());

    CreateInviteRequest {
        email: unique_email,
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        company_riseid: None,
        light: true,
    }
}

pub async fn authorize(rest_client: &RestApiClient<ExampleApiConfig>) {
    let resp = rest_client.get_auth_token().await;

    println!("Authorize response: {:?}", resp);
}

pub async fn create_invitation(rest_client: &RestApiClient<ExampleApiConfig>) {
    let resp = rest_client
        .create_invitation(create_invite_req())
        .await;

    println!("Create invitation response: {:?}", resp);
}

pub struct ExampleApiConfig;

#[async_trait::async_trait]
impl RestApiConfig for ExampleApiConfig {
    async fn get_api_url(&self) -> String {
        "https://b2b-api.staging-riseworks.io/v1".to_string()
    }

    async fn get_api_key(&self) -> String {
        std::env::var("RISE_API_KEY").unwrap_or_else(|_| "default_key".to_string())
    }

    async fn get_timeout(&self) -> Duration {
        Duration::from_secs(15)
    }

    async fn get_wallet_private_key(&self) -> String {
        std::env::var("WALLET_PRIVATE_KEY")
            .expect("WALLET_PRIVATE_KEY must be set in your environment or .env file")
    }
}