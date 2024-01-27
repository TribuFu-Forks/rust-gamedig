use crate::http::{HTTPSettings, HttpClient};
use crate::protocols::epic::{ClientTokenResponse, Request, Response, SessionFilter};
use crate::{GDResult, TimeoutSettings};
use std::net::{IpAddr, SocketAddr};

/*
/// Query a epic server.
#[inline]
pub fn query(address: &IpAddr, port: Option<u16>) -> GDResult<Response> {
    query_with_timeout(address, port, &None)
}

/// Query a epic server.
pub fn query_with_timeout(
    address: &IpAddr,
    port: Option<u16>,
    timeout_settings: &Option<TimeoutSettings>,
) -> GDResult<Response> {
    let address = &SocketAddr::new(*address, port.unwrap_or(3001));
    let mut client = HttpClient::new(
        address,
        timeout_settings,
        HTTPSettings {
            protocol: crate::http::Protocol::HTTP,
            hostname: None,
        },
    )?;

    Ok(response.into())
}
*/

pub fn get_client_oauth_token(client: &mut HttpClient, deployment_id: &str) -> GDResult<ClientTokenResponse> {
    let form_data = [
        ("grant_type", "client_credentials"),
        ("deployment_id", deployment_id),
    ];

    let response = client.post_form::<ClientTokenResponse>("/auth/v1/oauth/token", &form_data)?;

    Ok(response.into())
}

pub fn get_server_info(client: &mut HttpClient, deployment_id: &str) -> GDResult<Response> {
    let filter = SessionFilter::new("deployment_id", "eq", deployment_id);
    let request = Request::new().add_filter(filter);

    let path = format!("/matchmaking/v1/{}/filter", deployment_id);
    let response = client.post_json::<Response, Request>(&path, request)?;

    Ok(response.into())
}
