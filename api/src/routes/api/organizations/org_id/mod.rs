mod fundraising;
mod members;

use axum::{Extension, Json, extract::Path, middleware};
use color_eyre::eyre;
use mongodb::bson::oid::ObjectId;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::{AxumError, AxumResult},
    middlewares::{
        require_auth::UnauthorizedError, require_org_permissions::require_org_membership,
    },
    models::organization::Organization,
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_organization_by_id))
        .nest("/members", members::routes())
        .nest("/fundraising", fundraising::routes())
        .layer(middleware::from_fn(require_org_membership))
}

/// Get organization by id
#[utoipa::path(
    method(get),
    path = "/",
    params(
        ("org_id" = String, Path, description = "Organization id"),
    ),
    responses(
        (status = OK, description = "Success", body = Organization, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Organizations"
)]
async fn get_organization_by_id(
    Extension(state): Extension<AppState>,
    Path(org_id): Path<ObjectId>,
) -> AxumResult<Json<Organization>> {
    let org = state.store.organization.get_by_id(org_id).await?;

    if org.is_none() {
        return Err(AxumError::not_found(eyre::eyre!("Organization not found")));
    }

    Ok(Json(org.unwrap()))
}
