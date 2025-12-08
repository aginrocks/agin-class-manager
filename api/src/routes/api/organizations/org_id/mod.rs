mod fundraising;
mod members;
mod santa;

use axum::{
    Extension, Json,
    extract::{Path, Query},
    middleware,
};
use color_eyre::eyre;
use sea_orm::{EntityTrait, ModelTrait};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::{AxumError, AxumResult},
    middlewares::{
        require_auth::UnauthorizedError,
        require_org_permissions::{require_org_membership, requre_org_admin},
    },
    models::{
        org_members,
        organization::{self, OrgUser, PopulatedOrganization},
        user,
    },
    routes::api::{
        Success,
        organizations::{GetOrgQuery, OrganizationResponse},
    },
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

/// Get organization by id
#[utoipa::path(
    method(get),
    path = "/",
    params(
        ("org_id" = i64, Path, description = "Organization id"),
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
    Path(org_id): Path<i64>,
    Query(query): Query<GetOrgQuery>,
) -> AxumResult<Json<OrganizationResponse>> {
    let org = organization::Entity::find_by_id(org_id)
        .one(&state.sea_orm)
        .await?;

    if org.is_none() {
        return Err(AxumError::not_found(eyre::eyre!("Organization not found")));
    }

    let org = org.unwrap();

    if query.user_details {
        let users = org
            .find_related(user::Entity)
            .select_also(org_members::Entity)
            .all(&state.sea_orm)
            .await?;

        let members = users
            .into_iter()
            .filter_map(|(user, membership)| {
                Some(OrgUser {
                    id: user.id,
                    email: user.email,
                    name: user.name,
                    role: membership?.role,
                })
            })
            .collect();

        Ok(Json(OrganizationResponse::Populated(
            PopulatedOrganization {
                id: org.id,
                name: org.name,
                description: org.description,
                slug: org.slug,
                members,
                avatar_url: org.avatar_url,
                budget: org.budget,
            },
        )))
    } else {
        Ok(Json(OrganizationResponse::Basic(org)))
    }
}

/// Delete organization by id
#[utoipa::path(
    method(delete),
    path = "/",
    params(
        ("org_id" = i64, Path, description = "Organization id"),
    ),
    responses(
        (status = OK, description = "Success", body = Success, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Organizations"
)]
async fn delete_organization(
    Extension(state): Extension<AppState>,
    Path(_org_id): Path<i64>,
    Extension(organization): Extension<organization::Model>,
) -> AxumResult<Json<Success>> {
    organization.delete(&state.sea_orm).await?;
    Ok(Json(Success { success: true }))
}
