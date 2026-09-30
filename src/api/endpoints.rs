use http::Method;

#[derive(Clone, Debug)]
pub enum RestApiEndpoint {
    GetSiweMessage,
    ExecuteSiweAuth,
    Invite,
    /// `team_id` accepts a team/company RiseID, RiseAccount address or nanoid.
    TeamTalent { team_id: String },
}

impl From<&RestApiEndpoint> for String {
    fn from(item: &RestApiEndpoint) -> Self {
        let api_version = "v1";

        match item {
            RestApiEndpoint::GetSiweMessage | RestApiEndpoint::ExecuteSiweAuth => format!("{api_version}/auth/api/siwe"),
            RestApiEndpoint::Invite => {
                format!("{api_version}/invites")
            }
            RestApiEndpoint::TeamTalent { team_id } => {
                format!("{api_version}/teams/{team_id}/talent")
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
            RestApiEndpoint::TeamTalent { .. } => Method::GET,
        }
    }
}
