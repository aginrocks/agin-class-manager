mod create;
mod org_id;

use axum::{Extension, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::AxumResult,
    middlewares::require_auth::{UnauthorizedError, UserId},
    models::organization::Organization,
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_organizations))
        .merge(create::routes())
        .nest("/{org_id}", org_id::routes())
}

/// Get all organizations
#[utoipa::path(
    method(get),
    path = "/",
    responses(
        (status = OK, description = "Success", body = Vec<Organization>, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Organizations"
)]
async fn get_organizations(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
) -> AxumResult<Json<Vec<Organization>>> {
    let courses = state.store.organization.get_all(user_id.0).await?;

    Ok(Json(courses))
}
