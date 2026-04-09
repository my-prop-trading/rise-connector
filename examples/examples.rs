use rise_connector::api::api_client::{RestApiClient, RestApiConfig};
use std::time::Duration;
use tokio::time::Instant;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = ExampleApiConfig;
    let rise_api = RestApiClient::new(config);
    let instant = Instant::now();

    create_invitation(&rise_api).await;

    println!("elapsed time: {:?}", instant.elapsed());
}

pub async fn create_invitation(rest_client: &RestApiClient<ExampleApiConfig>) {
    let resp = rest_client
        .create_invitation(
            [
                "some_rise_id".to_string(),
                format!("test-{}@mailinator.com", Uuid::new_v4()),
            ]
            .to_vec(),
        )
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

    async fn get_company_rise_id(&self) -> String {
        std::env::var("COMPANY_RISE_ID")
            .expect("COMPANY_RISE_ID must be set in your environment or .env file")
    }

    async fn get_company_email(&self) -> String {
        std::env::var("COMPANY_EMAIL")
            .expect("COMPANY_EMAIL must be set in your environment or .env file")
    }

    async fn token_cache_enabled(&self) -> bool {
        false
    }
}
