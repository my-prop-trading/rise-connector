use http::Method;

#[derive(Clone, Copy, Debug)]
pub enum RestApiEndpoint {
    GetMessageToSign,
    AuthLogin,
    Invite,
}

impl From<&RestApiEndpoint> for String {
    fn from(item: &RestApiEndpoint) -> Self {
        let api_version = "v1";

        match item {
            RestApiEndpoint::GetMessageToSign | RestApiEndpoint::AuthLogin => format!("/auth/api/siwe"),
            RestApiEndpoint::Invite => {
                format!("/{api_version}/invites")
            }
        }
    }
}

impl RestApiEndpoint {
    pub fn get_http_method(&self) -> Method {
        match &self {
            RestApiEndpoint::AuthLogin => Method::POST,
            RestApiEndpoint::Invite => Method::POST,
            RestApiEndpoint::GetMessageToSign => Method::GET,
        }
    }
}
