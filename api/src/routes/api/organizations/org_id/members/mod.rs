mod add;

use axum::middleware;
use utoipa_axum::router::OpenApiRouter;

use crate::{middlewares::require_org_permissions::requre_org_admin, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/add", add::routes())
        .layer(middleware::from_fn(requre_org_admin))
}
