mod create;
mod org_id;

use axum::{Extension, Json, extract::Query};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::AxumResult,
    middlewares::require_auth::{UnauthorizedError, UserId},
    routes::api::organizations::org_id::{GetOrgQuery, OrganizationResponse},
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
    params (
         ("user-details" = Option<bool>, Query, description = "Include detailed user information for members"),
    ),
    responses(
        (status = OK, description = "Success", body = Vec<OrganizationResponse>, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Organizations"
)]
async fn get_organizations(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
    Query(query): Query<GetOrgQuery>,
) -> AxumResult<Json<Vec<OrganizationResponse>>> {
    let organizations = state.store.organization.get_all(user_id.0).await?;

    if query.user_details {
        let mut populated = Vec::new();
        for org in organizations {
            let pop = org.populate_users(state.clone()).await?;
            populated.push(OrganizationResponse::Populated(pop));
        }
        Ok(Json(populated))
    } else {
        let basic: Vec<OrganizationResponse> = organizations
            .into_iter()
            .map(|org| OrganizationResponse::Basic(org))
            .collect();
        Ok(Json(basic))
    }
}
