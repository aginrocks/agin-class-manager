mod create;
// mod org_id;

use axum::{Extension, Json, extract::Query};
use sea_orm::{ModelTrait, QueryFilter};
use serde::Deserialize;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::AxumResult,
    middlewares::require_auth::UnauthorizedError,
    models::{
        org_members, organization::{self, OrgUser, PopulatedOrganization}, user
    },
    // routes::api::organizations::org_id::{GetOrgQuery, OrganizationResponse},
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_organizations))
        .merge(create::routes())
    // .nest("/{org_id}", org_id::routes())
}

#[derive(Deserialize)]
pub struct GetOrgQuery {
    #[serde(default, rename = "user-details")]
    pub user_details: bool,
}

#[derive(serde::Serialize, utoipa::ToSchema)]
#[serde(untagged)]
pub enum OrganizationResponse {
    Basic(organization::Model),
    Populated(PopulatedOrganization),
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
    Extension(user): Extension<user::Model>,
    Extension(organizations): Extension<Vec<organization::Model>>,
    Query(query): Query<GetOrgQuery>,
) -> AxumResult<Json<Vec<OrganizationResponse>>> {
    dbg!("get_organizations");
    dbg!(&query.user_details);
    if query.user_details {
        let mut populated = Vec::new();
        for org in organizations {
            let users = org.find_related(user::Entity).select_with(org_members::Entity).all(&state.sea_orm).await?;



            let members = users.iter().map(|user| return OrgUser{
                id: user.id,
                email: user.email,
                name: user.name,
                role:
            })

            populated.push(OrganizationResponse::Populated(PopulatedOrganization { id: org.id, name: org.name, description: org.description, slug: org.slug, members: pop, avatar_url: (), budget: () }));
        }
        Ok(Json(populated))
    } else {
        let basic: Vec<OrganizationResponse> = organizations
            .into_iter()
            .map(|org| OrganizationResponse::Basic(org))
            .collect();
        Ok(Json(basic))
    }

    // Ok(Json(organizations))
}
