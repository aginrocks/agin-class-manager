mod create;

use axum::{Extension, Json};
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::{
    axum_error::AxumResult,
    middlewares::require_auth::UnauthorizedError,
    models::{
        fundraising::{self, FundraisingRes},
        organization,
        payers::{self, PayerRes},
        user,
    },
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    let create = create::routes();
    let router = OpenApiRouter::new().routes(routes!(get_fundraisings));

    create.merge(router)
}

/// Get all fundraisings for an organization
#[utoipa::path(
    method(get),
    path = "/",
    responses(
        (status = OK, description = "Success", body = Vec<FundraisingRes>, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Fundraisings"
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

        let mut total_amount: i64 = 0;

        let payers = users
            .into_iter()
            .filter_map(|(user, payer)| {
                let Some(payer) = payer.clone() else {
                    return None;
                };

                total_amount += payer.amount_to_pay;

                Some(PayerRes {
                    paid_amount: payer.paid_amount,
                    user_id: user.id,
                    amount_to_pay: payer.amount_to_pay,
                    comment: payer.comment,
                })
            })
            .collect();

        res.push(FundraisingRes {
            description: fundraising.description,
            id: fundraising.id,
            name: fundraising.name,
            total_amount: total_amount,
            start_date: fundraising.start_date,
            end_date: fundraising.end_date,
            organization_id: fundraising.organization_id,
            payers,
        });
    }

    Ok(Json(res))
}
