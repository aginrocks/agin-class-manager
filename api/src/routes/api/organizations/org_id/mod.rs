mod fundraising;
mod members;
mod santa;

use axum::{
    Extension, Json,
    extract::{Path, Query},
    middleware,
};
use color_eyre::eyre;
use mongodb::bson::oid::ObjectId;
use serde::Deserialize;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::{AxumError, AxumResult},
    middlewares::{
        require_auth::UnauthorizedError,
        require_org_permissions::{require_org_membership, requre_org_admin},
    },
    models::organization::{Organization, PopulatedOrganization},
    routes::api::Success,
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    let admin = OpenApiRouter::new()
        .routes(routes!(delete_organization))
        .layer(middleware::from_fn(requre_org_admin))
        .layer(middleware::from_fn(require_org_membership));

    let user = OpenApiRouter::new()
        .routes(routes!(get_organization_by_id))
        .nest("/members", members::routes())
        .nest("/fundraising", fundraising::routes())
        .nest("/santa", santa::routes())
        .layer(middleware::from_fn(require_org_membership));

    admin.merge(user)
}

#[derive(Deserialize)]
pub struct GetOrgQuery {
    #[serde(default, rename = "user-details")]
    pub user_details: bool,
}

#[derive(serde::Serialize, utoipa::ToSchema)]
#[serde(untagged)]
pub enum OrganizationResponse {
    Basic(Organization),
    Populated(PopulatedOrganization),
}

/// Get organization by id
#[utoipa::path(
    method(get),
    path = "/",
    params(
        ("org_id" = String, Path, description = "Organization id"),
        ("user-details" = Option<bool>, Query, description = "Include detailed user information for members"),
    ),
    responses(
        (status = OK, description = "Success", body = OrganizationResponse, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Organizations"
)]
async fn get_organization_by_id(
    Extension(state): Extension<AppState>,
    Path(org_id): Path<ObjectId>,
    Query(query): Query<GetOrgQuery>,
) -> AxumResult<Json<OrganizationResponse>> {
    let org = state.store.organization.get_by_id(org_id).await?;

    if org.is_none() {
        return Err(AxumError::not_found(eyre::eyre!("Organization not found")));
    }

    let org = org.unwrap();

    if query.user_details {
        let populated = org.populate_users(state).await?;
        Ok(Json(OrganizationResponse::Populated(populated)))
    } else {
        Ok(Json(OrganizationResponse::Basic(org)))
    }
}

/// Delete organization by id
#[utoipa::path(
    method(delete),
    path = "/",
    params(
        ("org_id" = String, Path, description = "Organization id"),
    ),
    responses(
        (status = OK, description = "Success", body = Success, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Organizations"
)]
async fn delete_organization(
    Extension(state): Extension<AppState>,
    Path(org_id): Path<ObjectId>,
) -> AxumResult<Json<Success>> {
    state.store.organization.delete(org_id).await?;

    Ok(Json(Success { success: true }))
}
