use alloy::signers::Signer;
use alloy::signers::local::PrivateKeySigner;
use error_chain::bail;
use flurl::{FlUrl, FlUrlResponse};
use http::{Method, StatusCode};
use my_logger::LogEventCtx;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::sync::RwLock;
use std::time::Duration;

use crate::api::endpoints::RestApiEndpoint;
use crate::api::errors::Error;
use crate::api::models::{
    ApiResponse, CreateInviteRequest, CreateInviteResponse, SiweLoginRequest, SiweLoginResponse,
    SiweMessageResponse,
};

#[async_trait::async_trait]
pub trait RestApiConfig {
    async fn get_api_url(&self) -> String;
    async fn get_timeout(&self) -> Duration;
    async fn get_wallet_private_key(&self) -> String;
    async fn get_company_rise_id(&self) -> String;
    async fn get_company_email(&self) -> String;
    async fn token_cache_enabled(&self) -> bool;
}

pub struct RestApiClient<C: RestApiConfig> {
    config: C,
    token_cache: RwLock<Option<String>>,
}

impl<C: RestApiConfig> RestApiClient<C> {
    pub fn new(config: C) -> Self {
        Self {
            config,
            token_cache: RwLock::new(None),
        }
    }

    pub async fn create_invitation(
        &self,
        invite_list: Vec<String>,
    ) -> Result<ApiResponse<CreateInviteResponse>, Error> {
        let token = self.get_token().await;

        let Ok(token) = token else {
            return Err(format!("Failed to get token: {:?}", token.unwrap_err()).into());
        };

        let invite_request = CreateInviteRequest {
            invite_list,
            anonymous: false,
            company_riseid: self.config.get_company_rise_id().await,
            role: crate::api::models::Role::Contractor,
        };

        let res = self.send_invitation(&invite_request, token).await;

        match res {
            Ok(response) => Ok(response),
            Err(e) => {
                let err_msg = e.to_string();
                if err_msg.contains("401") || err_msg.contains("403") || 
                   err_msg.contains("Unauthorized") || err_msg.contains("Forbidden") 
                {
                    let mut write_guard = self
                        .token_cache
                        .write()
                        .map_err(|e| format!("Write lock poisoned: {}", e))?;
                    *write_guard = None;
        
                    let token = self.get_token().await?;

                    self.send_invitation(&invite_request, token).await
                } else {
                    Err(e)
                }
            }
        }
    }

    async fn create_auth_token(&self) -> Result<SiweLoginResponse, Error> {
        let raw_key = self.config.get_wallet_private_key().await;
        my_logger::LOGGER.write_debug(
            "create_auth_token",
            format!("raw_key: {}", raw_key),
            LogEventCtx::new(),
        );
        let wallet: PrivateKeySigner = raw_key
            .parse::<PrivateKeySigner>()
            .map_err(|e| Error::RestError(format!("Failed to initialize signer: {}", e)))?;
        let wallet_address = format!("{}", wallet.address());

        let body: Option<&()> = None;
        let message_data: ApiResponse<SiweMessageResponse> = self
            .send_deserialized(
                RestApiEndpoint::GetSiweMessage,
                body,
                Some(self.build_query_string(vec![
                    ("wallet", wallet_address.as_str()),
                    (
                        "impersonate",
                        self.config.get_company_email().await.as_str(),
                    ),
                    ("rise_id", self.config.get_company_rise_id().await.as_str()),
                ])),
                vec![],
            )
            .await?;

        if message_data.data.wallet.to_lowercase() != wallet_address.to_lowercase() {
            return Err("Wallet mismatch from API".into());
        }

        let signature = wallet
            .sign_message(&message_data.data.message.as_bytes())
            .await
            .map_err(|e| format!("Signing failed: {}", e))?;

        let login_req = SiweLoginRequest {
            wallet: wallet_address,
            message: message_data.data.message,
            signature: format!("0x{}", alloy::hex::encode(signature.as_bytes())),
        };

        let login_data: ApiResponse<SiweLoginResponse> = self
            .send_deserialized(
                RestApiEndpoint::ExecuteSiweAuth,
                Some(&login_req),
                None,
                vec![],
            )
            .await?;

        Ok(login_data.data)
    }

    async fn get_token(&self) -> Result<String, Error> {
        let cache_enabled = self.config.token_cache_enabled().await;

        if cache_enabled {
            if let Some(token) = self.read_token_cache().map_err(Error::from)? {
                return Ok(token);
            }
        }

        let login_response = self.create_auth_token().await?;

        if cache_enabled {
            let mut write_guard = self
                .token_cache
                .write()
                .map_err(|e| format!("Write lock poisoned: {}", e))?;
            *write_guard = Some(login_response.token.clone());
        }

        Ok(login_response.token)
    }

    async fn send_invitation(
        &self,
        invite: &CreateInviteRequest,
        token: String,
    ) -> Result<ApiResponse<CreateInviteResponse>, Error> {
        self.send_deserialized(
            RestApiEndpoint::Invite,
            Some(invite),
            None,
            vec![("Authorization", format!("Bearer {token}").as_str())],
        )
        .await
    }

    async fn send_deserialized<R: Serialize + Debug, T: DeserializeOwned + Debug>(
        &self,
        endpoint: RestApiEndpoint,
        request: Option<&R>,
        query_string: Option<String>,
        extra_headers: Vec<(&str, &str)>,
    ) -> Result<T, Error> {
        if std::env::var("DEBUG").is_ok() {
            println!("execute send_deserialized: {:?} {:?}", endpoint, request);
        }

        let timeout = self.config.get_timeout().await;
        let response = tokio::time::timeout(
            timeout,
            self.send_flurl_deserialized(&endpoint, request, query_string, extra_headers),
        )
        .await;

        let Ok(response) = response else {
            let msg = format!(
                "Failed {:?} {:?}: Timeout",
                endpoint.get_http_method(),
                endpoint
            );
            return Err(msg.into());
        };

        response
    }

    fn build_full_url(
        &self,
        base_url: &str,
        endpoint: &RestApiEndpoint,
        query_string: Option<String>,
    ) -> String {
        let endpoint_str = String::from(endpoint);

        if let Some(query_string) = query_string {
            format!("{base_url}{endpoint_str}?{query_string}")
        } else {
            format!("{base_url}{endpoint_str}")
        }
    }

    async fn send_flurl_deserialized<R: Serialize + Debug, T: DeserializeOwned + Debug>(
        &self,
        endpoint: &RestApiEndpoint,
        request: Option<&R>,
        query_string: Option<String>,
        extra_headers: Vec<(&str, &str)>,
    ) -> Result<T, Error> {
        let response = self
            .send_flurl(endpoint, request, query_string, extra_headers)
            .await?;
        let result: Result<T, _> = serde_json::from_str(&response);

        let Ok(body) = result else {
            let msg = format!(
                "Failed to deserialize. Err: {:?} Url: {:?} {:?}. Request: {:?}. Body: {}",
                result.unwrap_err(),
                endpoint.get_http_method(),
                String::from(endpoint),
                request,
                response
            );
            return Err(msg.into());
        };

        Ok(body)
    }

    async fn send_flurl<R: Serialize + Debug>(
        &self,
        endpoint: &RestApiEndpoint,
        request: Option<&R>,
        query_string: Option<String>,
        extra_headers: Vec<(&str, &str)>,
    ) -> Result<String, Error> {
        let mut request_json = None;

        if let Some(request) = request {
            let body = serde_json::to_string(request)?;
            request_json = Some(body.clone());
        }

        let request_bytes: Option<Vec<u8>> = if let Some(request) = request {
            Some(serde_json::to_string(request)?.into_bytes())
        } else {
            None
        };
        let (flurl, url) = self
            .build_flurl::<()>(endpoint, query_string, extra_headers)
            .await?;
        let http_method = endpoint.get_http_method();

        let result = if http_method == Method::GET {
            flurl.get().await
        } else if http_method == Method::POST {
            flurl.post(request_bytes).await
        } else if http_method == Method::PUT {
            flurl.put(request_bytes).await
        } else if http_method == Method::PATCH {
            flurl.patch(request_bytes).await
        } else if http_method == Method::DELETE {
            flurl.delete().await
        } else {
            panic!("not implemented");
        };

        let Ok(resp) = result else {
            return Err(format!(
                "FlUrl failed to receive_body: Url: {}. Request: {:?}. {:?}",
                url,
                request_json,
                result.unwrap_err()
            )
            .into());
        };

        handle_flurl_text(resp, &request_json, &url, endpoint.get_http_method()).await
    }

    pub async fn build_flurl<R: Serialize>(
        &self,
        endpoint: &RestApiEndpoint,
        query_string: Option<String>,
        extra_headers: Vec<(&str, &str)>,
    ) -> Result<(FlUrl, String), Error> {
        let base_url = self.config.get_api_url().await;

        let url = self.build_full_url(&base_url, endpoint, query_string);
        let flurl = FlUrl::new(&url).set_timeout(self.config.get_timeout().await);
        let flurl = self.add_headers(flurl, extra_headers).await;

        Ok((flurl, url))
    }

    async fn add_headers(&self, flurl: FlUrl, extra_headers: Vec<(&str, &str)>) -> FlUrl {
        let json_content_str = "application/json";

        let mut flurl = flurl
            .with_header("Content-Type", json_content_str)
            .with_header("Accept", json_content_str);
        for (key, value) in extra_headers {
            flurl = flurl.with_header(key, value);
        }

        flurl
    }

    pub fn build_query_string(&self, params: Vec<(&str, &str)>) -> String {
        serde_urlencoded::to_string(params).unwrap_or_default()
    }

    fn read_token_cache(&self) -> Result<Option<String>, String> {
        let guard = self
            .token_cache
            .read()
            .expect("short critical section, can't poison lock");

        Ok(guard.clone())
    }
}

async fn handle_flurl_text(
    response: FlUrlResponse,
    request_json: &Option<String>,
    request_url: &str,
    request_method: Method,
) -> Result<String, Error> {
    let status_code = StatusCode::from_u16(response.get_status_code()).unwrap();
    let result = response.receive_body().await;

    let Ok(body_bytes) = result else {
        return Err(format!("FlUrl failed to receive_body: {:?}", result.unwrap_err()).into());
    };

    let body_str = String::from_utf8(body_bytes).unwrap();

    match status_code {
        StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(body_str),
        StatusCode::INTERNAL_SERVER_ERROR => {
            bail!(format!(
                "Internal Server Error. Url: {request_method:?} {request_url}. Request: {:?}. Response: {}",
                request_json, body_str,
            ));
        }
        StatusCode::SERVICE_UNAVAILABLE => {
            bail!(format!(
                "Service Unavailable. Url: {request_method:?} {request_url}. Request: {:?}. Response: {}",
                request_json, body_str
            ));
        }
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
            bail!(format!(
                "Unauthorized or forbidden. Url: {request_method:?} {request_url}. Request: {:?}. Response: {}",
                request_json, body_str
            ));
        }
        StatusCode::BAD_REQUEST => {
            let error = body_str;
            bail!(format!(
                "Received bad request status. Url: {request_method:?} {request_url}. Request: {request_json:?}. Response: {error:?}"
            ));
        }
        code => {
            let error = body_str;
            bail!(format!(
                "Received response code: {code:?}. Url: {request_method:?} {request_url}. Request: {request_json:?} Response: {error:?}"
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn works() {}
}
