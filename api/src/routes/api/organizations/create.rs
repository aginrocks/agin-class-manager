use axum::{Extension, Json, extract::State};
use axum_valid::Valid;
use regex::Regex;
use sea_orm::{ActiveValue::Set, EntityTrait};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::AxumResult,
    middlewares::require_auth::UnauthorizedError,
    models::{
        org_members::{self, Membership},
        organization::{self, MutableOrganization, OrganizationRes},
        user,
    },
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(create_organization))
}

fn sanitize(s: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9]+").unwrap();
    let lower = s.to_lowercase();
    let replaced = re.replace_all(&lower, "_");
    replaced.trim_matches('_').to_string()
}

/// Create a new organization
#[utoipa::path(
    method(post),
    path = "/",
    request_body = MutableOrganization,
    responses(
        (status = OK, description = "Success", body = OrganizationRes),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Organizations"
)]
async fn create_organization(
    Extension(user): Extension<user::Model>,
    State(state): State<AppState>,
    Valid(body): Valid<Json<MutableOrganization>>,
) -> AxumResult<Json<OrganizationRes>> {
    let slug = match &body.slug {
        Some(slug) => slug.to_owned(),
        None => sanitize(&body.name),
    };

    dbg!(body.description.clone());

    let organization_model = organization::ActiveModel {
        name: Set(body.name.clone()),
        description: Set(body.description.clone()),
        avatar_url: Set(body.avatar_url.clone()),
        slug: Set(slug),
        budget: Set(0),
        ..Default::default()
    };

    let organization = organization::Entity::insert(organization_model)
        .exec_with_returning(&state.sea_orm)
        .await?;

    let members = org_members::ActiveModel {
        user_id: Set(user.id),
        org_id: Set(organization.id.clone()),
        role: Set(org_members::OrganizationRole::Admin),
    };

    let org_members = org_members::Entity::insert(members)
        .exec_with_returning(&state.sea_orm)
        .await?;

    let membership = Membership {
        role: org_members.role,
        user_id: user.id,
    };

    Ok(Json(OrganizationRes {
        id: organization.id,
        name: organization.name,
        slug: organization.slug,
        description: organization.description,
        avatar_url: organization.avatar_url,
        budget: organization.budget,
        members: vec![membership],
    }))
}
