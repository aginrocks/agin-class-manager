use axum::{Extension, Json, extract::State, middleware};
use axum_valid::Valid;
use sea_orm::{ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::{
    axum_error::AxumResult,
    middlewares::{require_auth::UnauthorizedError, require_org_permissions::requre_org_admin},
    models::{
        fundraising::{self, FundraisingRes, MutableFundraising, Payer},
        organization, payers, user,
    },
    routes::api::CreateSuccess,
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
        total_amount: Set(body.total_amount),
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

/// Get all fundraisings for an organization
#[utoipa::path(
    method(get),
    path = "/",
    responses(
        (status = OK, description = "Success", body = Vec<FundraisingRes>, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Fundraising"
)]
async fn get_fundraisings(
    Extension(state): Extension<AppState>,
    Extension(org_data): Extension<organization::Model>,
) -> AxumResult<Json<Vec<FundraisingRes>>> {
    let mut res = vec![];

    let fundraisings = fundraising::Entity::find()
        .filter(fundraising::Column::OrganizationId.eq(org_data.id))
        .all(&state.sea_orm)
        .await?;

    for fundraising in fundraisings {
        let users = fundraising
            .find_related(user::Entity)
            .select_also(payers::Entity)
            .all(&state.sea_orm)
            .await?;

        let payers = users
            .into_iter()
            .filter_map(|(user, payer)| {
                Some(Payer {
                    paid_amount: payer?.paid_amount,
                    user_id: user.id,
                })
            })
            .collect();

        res.push(FundraisingRes {
            description: fundraising.description,
            id: fundraising.id,
            name: fundraising.name,
            total_amount: fundraising.total_amount,
            start_date: fundraising.start_date,
            end_date: fundraising.end_date,
            organization_id: fundraising.organization_id,
            payers,
        });
    }

    Ok(Json(res))
}
