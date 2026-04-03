use rise_connector::api::{
    api_client::{RestApiClient, RestApiConfig},
    models::CreateInviteRequest,
};
use std::time::Duration;
use tokio::time::Instant;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = ExampleApiConfig;
    let rise_api = RestApiClient::new(config);
    let instant = Instant::now();

    let login_response = authorize(&rise_api)
        .await
        .unwrap_or_else(|e| panic!("Authorization failed: {}", e));

    println!("token: {:?}", login_response.token);
    create_invitation(&rise_api, login_response.token).await;

    println!("elapsed time: {:?}", instant.elapsed());
}

pub fn create_invite_req() -> CreateInviteRequest {
    let unique_email = format!("test-{}@mailinator.com", Uuid::new_v4());

    CreateInviteRequest {
        company_riseid: "some_company_riseid".to_string(),
        invite_list: ["some_rise_id".to_string(), unique_email].to_vec(),
        anonymous: false,
        role: rise_connector::api::models::Role::Contractor
    }
}

pub async fn authorize(rest_client: &RestApiClient<ExampleApiConfig>) -> Result<rise_connector::api::models::SiweLoginResponse, rise_connector::api::errors::Error> {
    let resp = rest_client.get_auth_token().await;

    println!("Authorize response: {:?}", resp);
    resp
}

pub async fn create_invitation(rest_client: &RestApiClient<ExampleApiConfig>, token: String) {
    let resp = rest_client
        .create_invitation(create_invite_req(), token)
        .await;

    println!("Create invitation response: {:?}", resp);
}

pub struct ExampleApiConfig;

#[async_trait::async_trait]
impl RestApiConfig for ExampleApiConfig {
    async fn get_api_url(&self) -> String {
        "https://b2b-api.riseworks.io/".to_string()
    }

    async fn get_timeout(&self) -> Duration {
        Duration::from_secs(15)
    }

    async fn get_wallet_private_key(&self) -> String {
        std::env::var("WALLET_PRIVATE_KEY")
            .expect("WALLET_PRIVATE_KEY must be set in your environment or .env file")
    }
}
