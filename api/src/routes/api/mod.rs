mod health;
mod login;
mod logout;
mod organizations;
mod user;

use axum::middleware;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;

use crate::{middlewares::require_auth::require_auth, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    let auth = OpenApiRouter::new()
        .nest("/user", user::routes())
        .nest("/organizations", organizations::routes())
        .layer(middleware::from_fn(require_auth));

    let public = OpenApiRouter::new()
        .nest("/health", health::routes())
        .nest("/login", login::routes())
        .nest("/logout", logout::routes());

    auth.merge(public)
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"error": "Not Found"}))]
pub struct NotFoundError {
    error: String,
}

#[derive(Serialize, ToSchema)]
pub struct GenericError {
    error: String,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"success": true,"id": "60c72b2f9b1d8c001c8e4f5a"}))]
pub struct CreateSuccess {
    success: bool,
    id: String,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"success": true}))]
pub struct Success {
    success: bool,
}
