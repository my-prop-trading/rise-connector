use http::Method;

#[derive(Clone, Copy, Debug)]
pub enum RestApiEndpoint {
    GetSiweMessage,
    ExecuteSiweAuth,
    Invite,
}

impl From<&RestApiEndpoint> for String {
    fn from(item: &RestApiEndpoint) -> Self {
        let api_version = "v1";

        match item {
            RestApiEndpoint::GetSiweMessage | RestApiEndpoint::ExecuteSiweAuth => format!("{api_version}/auth/api/siwe"),
            RestApiEndpoint::Invite => {
                format!("{api_version}/invites")
            }
        }
    }
}

impl RestApiEndpoint {
    pub fn get_http_method(&self) -> Method {
        match &self {
            RestApiEndpoint::ExecuteSiweAuth => Method::POST,
            RestApiEndpoint::Invite => Method::POST,
            RestApiEndpoint::GetSiweMessage => Method::GET,
        }
    }
}
