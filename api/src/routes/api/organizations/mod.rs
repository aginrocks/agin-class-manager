mod create;

use axum::{Extension, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::AxumResult, middlewares::require_auth::UnauthorizedError,
    models::organization::Organization, state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_courses))
        .nest("/create", create::routes())
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
async fn get_courses(Extension(state): Extension<AppState>) -> AxumResult<Json<Vec<Organization>>> {
    let courses = state.store.organization.get_all().await?;

    Ok(Json(courses))
}
