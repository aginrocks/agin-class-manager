use axum::{Extension, Json, extract::State};
use axum_valid::Valid;
use color_eyre::eyre::{self, Context, ContextCompat};
use regex::Regex;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::{AxumError, AxumResult},
    middlewares::require_auth::{UnauthorizedError, UserId},
    models::{
        organization::{MutableOrganization, Organization, PartialOrganization},
        user::{Membership, OrganizationRole},
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
        (status = OK, description = "Success", body = Organization),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Organizations"
)]
async fn create_organization(
    Extension(user_id): Extension<UserId>,
    State(state): State<AppState>,
    Valid(body): Valid<Json<MutableOrganization>>,
) -> AxumResult<Json<Organization>> {
    let slug = match &body.slug {
        Some(slug) => slug.to_owned(),
        None => sanitize(&body.name),
    };

    let already_exists = state.store.organization.get_by_slug(&slug).await?;

    if already_exists.is_some() {
        return Err(AxumError::forbidden(eyre::eyre!(
            "Organization with this slug already exists"
        )));
    }

    let name = body.name.clone();
    let description = body.description.clone();
    let avatar_url = body.avatar_url.clone();
    let members = vec![Membership {
        user_id: user_id.0,
        role: OrganizationRole::Admin,
    }];
    let budget = 0;

    let organization = PartialOrganization {
        name: name.clone(),
        description: description.clone(),
        slug: slug.clone(),
        avatar_url: avatar_url.clone(),
        members: members.clone(),
        budget,
    };

    let inserted_org = state
        .store
        .organization
        .partial_collection
        .insert_one(organization)
        .await
        .wrap_err("Failed to create organization")?;

    let id = inserted_org
        .inserted_id
        .as_object_id()
        .wrap_err("Failed to fetch organization ID")?;

    Ok(Json(Organization {
        id,
        name,
        slug,
        description,
        avatar_url,
        budget,
        members,
    }))
}
