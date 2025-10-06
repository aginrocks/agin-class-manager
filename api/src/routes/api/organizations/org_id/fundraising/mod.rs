use axum::{Extension, Json, extract::State, middleware};
use axum_valid::Valid;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::{
    axum_error::AxumResult,
    middlewares::{
        require_auth::UnauthorizedError,
        require_org_permissions::{OrganizationData, requre_org_admin},
    },
    models::fundraising::{Fundraising, MutableFundraising, PartialFundraising},
    routes::{self, api::CreateSuccess},
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    let admin = OpenApiRouter::new()
        .routes(routes!(create_fundraising))
        .layer(middleware::from_fn(requre_org_admin));
    let user = OpenApiRouter::new().routes(routes!(get_fundraisings));

    admin.merge(user)
}

/// Create a new fundraising
#[utoipa::path(
    method(post),
    path = "/",
    request_body = MutableFundraising,
    responses(
        (status = OK, description = "Success", body = CreateSuccess),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Fundraising"
)]
#[axum::debug_handler]
async fn create_fundraising(
    State(state): State<AppState>,
    Extension(org_data): Extension<OrganizationData>,
    Valid(body): Valid<Json<MutableFundraising>>,
) -> AxumResult<Json<CreateSuccess>> {
    if body
        .payers
        .iter()
        .any(|p| !org_data.0.members.iter().any(|m| m.user_id == p.user_id))
    {
        return Err(crate::axum_error::AxumError::forbidden(
            color_eyre::eyre::eyre!("All payers must be members of the organization"),
        ));
    };

    let fundraising = state
        .store
        .fundraising
        .create(PartialFundraising {
            description: body.description.clone(),
            name: body.name.clone(),
            total_amount: body.total_amount,
            end_date: body.end_date.map(|d| d.into()),
            start_date: body.start_date.map(|d| d.into()),
            payers: body.payers.clone(),
            organization_id: org_data.0.id,
        })
        .await?;

    Ok(Json(CreateSuccess {
        success: true,
        id: fundraising.id.to_string(),
    }))
}

/// Get all fundraisings for an organization
#[utoipa::path(
    method(get),
    path = "/",
    responses(
        (status = OK, description = "Success", body = Vec<Fundraising>, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Fundraising"
)]
async fn get_fundraisings(
    Extension(state): Extension<AppState>,
    Extension(org_data): Extension<OrganizationData>,
) -> AxumResult<Json<Vec<Fundraising>>> {
    let fundraisings = state
        .store
        .fundraising
        .get_by_organization(&org_data.0.id)
        .await?;

    Ok(Json(fundraisings))
}
