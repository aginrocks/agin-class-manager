use axum::{
    Extension,
    extract::State,
    http::{HeaderMap, header::SET_COOKIE},
    response::{IntoResponse, Redirect, Response},
};
use color_eyre::eyre::eyre;
use sea_orm::ModelTrait;
use tower_sessions::Session;
use tracing::{error, info};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{axum_error::AxumResult, models::token, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(log_out))
}

/// Log out
///
/// Clears the user session, removes the OIDC 'id' cookie, and redirects to the OIDC end-session endpoint.
#[utoipa::path(method(get), path = "/", tag = "Auth")]
async fn log_out(
    State(state): State<AppState>,
    Extension(token): Extension<token::Model>,
    session: Session,
) -> AxumResult<Response> {
    // token
    //     .delete(&state.sea_orm)
    //     .await
    //     .map_err(|_| eyre!("Couldn't delete token"))?;

    if let Err(e) = session.flush().await {
        error!("Failed to flush session: {}", e);
    }

    if let Err(e) = session.delete().await {
        error!("Failed to delete session: {}", e);
    } else {
        info!("Session cleared successfully");
    }

    let mut headers = HeaderMap::new();

    headers.insert(
        SET_COOKIE,
        "id=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; SameSite=Lax"
            .parse()
            .unwrap(),
    );

    headers.insert(
        SET_COOKIE,
        "tower.sid=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; SameSite=Lax"
            .parse()
            .unwrap(),
    );

    let end_session_url = format!(
        "{}/end-session/",
        state.settings.oidc.issuer.as_str().trim_end_matches('/')
    );

    info!("Redirecting to OIDC logout: {}", end_session_url);

    let mut response = Redirect::to(&end_session_url).into_response();
    response.headers_mut().extend(headers);

    Ok(response)
}
