use axum::{Extension, Json, extract::State};
use axum_valid::Valid;
use color_eyre::eyre::{self, Context, ContextCompat};
use fred::rustls::pki_types::alg_id;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::{AxumError, AxumResult},
    middlewares::require_auth::{UnauthorizedError, UserId},
    models::{
        organization::{MutableOrganization, PartialOrganization},
        user::{Membership, OrganizationRole},
    },
    routes::api::CreateSuccess,
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(create_organization))
}

/// Create a new organization
#[utoipa::path(
    method(post),
    path = "/",
    request_body = MutableOrganization,
    responses(
        (status = OK, description = "Success", body = CreateSuccess),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Organizations"
)]
async fn create_organization(
    Extension(user_id): Extension<UserId>,
    State(state): State<AppState>,
    Valid(body): Valid<Json<MutableOrganization>>,
) -> AxumResult<Json<CreateSuccess>> {
    // let already_exists = state
    //     .database
    //     .collection::<Organization>("organizations")
    //     .find_one(doc! { "slug": &body.slug })
    //     .await?;

    let already_exists = state.store.organization.get_by_slug(&body.slug).await?;

    if already_exists.is_some() {
        return Err(AxumError::forbidden(eyre::eyre!(
            "Organization with this slug already exists"
        )));
    }

    let organization = PartialOrganization {
        name: body.name.clone(),
        description: body.description.clone(),
        slug: body.slug.clone(),
        avatar_url: body.avatar_url.clone(),
        members: vec![Membership {
            user_id: user_id.0,
            role: OrganizationRole::Admin,
        }],
        budget: 0,
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
        .wrap_err("Failed to fetch organization ID")?
        .to_string();

    Ok(Json(CreateSuccess { success: true, id }))
}
