use axum::{
    extract::State,
    http::{HeaderMap, header::SET_COOKIE},
    response::{IntoResponse, Redirect, Response},
};
use tower_sessions::Session;
use tracing::{error, info};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{axum_error::AxumResult, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(log_out))
}

/// Log out
///
/// Clears the user session, removes the OIDC 'id' cookie, and redirects to the OIDC end-session endpoint.
#[utoipa::path(method(get), path = "/", tag = "Auth")]
async fn log_out(State(state): State<AppState>, session: Session) -> AxumResult<Response> {
    // Clear the session from Redis
    if let Err(e) = session.flush().await {
        error!("Failed to flush session: {}", e);
    }

    if let Err(e) = session.delete().await {
        error!("Failed to delete session: {}", e);
    } else {
        info!("Session cleared successfully");
    }

    // Create response with cookie removal
    let mut headers = HeaderMap::new();

    // Clear the OIDC 'id' cookie by setting it to expire immediately
    headers.insert(
        SET_COOKIE,
        "id=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; SameSite=Lax"
            .parse()
            .unwrap(),
    );

    // Also clear any session cookie that might exist
    headers.insert(
        SET_COOKIE,
        "tower.sid=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; SameSite=Lax"
            .parse()
            .unwrap(),
    );

    // Construct the OIDC end-session URL using the issuer from settings
    let end_session_url = format!(
        "{}/end-session/",
        state.settings.oidc.issuer.as_str().trim_end_matches('/')
    );

    info!("Redirecting to OIDC logout: {}", end_session_url);

    // Create redirect response with cookie clearing headers
    let mut response = Redirect::to(&end_session_url).into_response();
    response.headers_mut().extend(headers);

    Ok(response)
}
