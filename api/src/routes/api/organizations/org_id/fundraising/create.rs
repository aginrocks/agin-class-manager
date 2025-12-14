use axum::{Extension, Json, extract::State, middleware};
use axum_valid::Valid;
use sea_orm::{ActiveValue::Set, EntityTrait, ModelTrait};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::AxumResult,
    middlewares::{require_auth::UnauthorizedError, require_org_permissions::requre_org_admin},
    models::{
        fundraising::{self, MutableFundraising},
        organization, user,
    },
    routes::api::CreateSuccess,
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create_fundraising))
        .layer(middleware::from_fn(requre_org_admin))
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
    tag = "Fundraisings"
)]
#[axum::debug_handler]
pub async fn create_fundraising(
    State(state): State<AppState>,
    Extension(org_data): Extension<organization::Model>,
    Valid(body): Valid<Json<MutableFundraising>>,
) -> AxumResult<Json<CreateSuccess>> {
    let users = org_data
        .find_related(user::Entity)
        .all(&state.sea_orm)
        .await?;

    if body
        .payers
        .iter()
        .any(|p| !users.iter().any(|u| u.id == p.user_id))
    {
        return Err(crate::axum_error::AxumError::forbidden(
            color_eyre::eyre::eyre!("All payers must be members of the organization"),
        ));
    };

    let fundraising_model = fundraising::ActiveModel {
        description: Set(body.description.clone()),
        name: Set(body.name.clone()),
        end_date: Set(body.end_date.map(|d| d.into())),
        start_date: Set(body.start_date.map(|d| d.into())),
        organization_id: Set(org_data.id),
        ..Default::default()
    };

    let fundraising = fundraising::Entity::insert(fundraising_model)
        .exec_with_returning(&state.sea_orm)
        .await?;

    Ok(Json(CreateSuccess {
        success: true,
        id: fundraising.id,
    }))
}
